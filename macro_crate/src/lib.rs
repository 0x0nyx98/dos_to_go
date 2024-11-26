extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_attribute]
pub fn entrypoint(_: TokenStream, item: TokenStream) -> TokenStream {
    match syn::parse::<syn::ItemFn>(item.clone()) {
        Ok(f) => {
            let func_name = f.sig.ident;
            let mut wrap = TokenStream::from(quote!(
                #[no_mangle]
                fn _dtg_entrypoint() { 
                    let code = #func_name();

                    exit(code);
                }

                 #[panic_handler]
                fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
                    unsafe {
                        asm!(
                            "mov ax, 0x4C80",
                            "int 0x21"
                        );
                    }
                    loop {}
                }
            ));
            wrap.extend(item);
            wrap
        }
        Err(e) => {
            let mut i = item.clone();
            i.extend(TokenStream::from(e.into_compile_error()));
            i
        }
    }
}