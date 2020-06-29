//! Custom derive macros for traits in the armbf crate.
//!
//! NOTE: There's still quite a lot of code reuse in this; no way around that?

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

/// Boilerplate code for parsing derive macro input.
macro_rules! get_tokenstream { ($input:expr, $impl:ident) => {
    $impl(&syn::parse_derive_input(&$input.to_string()).unwrap())
        .parse().unwrap()
}}


/* 
 * Derive macros for traits representing bitfields on ARM instructions.
 */

#[proc_macro_derive(InstBits)]
pub fn derive_inst_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_inst_common);
}
fn impl_inst_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl InstBits for #name {
            #[inline(always)]
            fn cond(&self) -> u32 {
                (self.0 & 0b1111_0000_0000_0000_0000_0000_0000_0000) >> 28
            }
            #[inline(always)]
            fn group(&self) -> u32 {
                (self.0 & 0b0000_1110_0000_0000_0000_0000_0000_0000) >> 25
            }
        }
    }
}


#[proc_macro_derive(DpBits)]
pub fn derive_dp_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_common);
}
fn impl_dp_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpBits for #name {
            #[inline(always)]
            fn opcd(&self) -> u32 {
                (self.0 & 0b0000_0001_1110_0000_0000_0000_0000_0000) >> 21
            }
            #[inline(always)]
            fn s(&self) -> bool {
                (self.0 & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0
            }
        }
    }
}


#[proc_macro_derive(LsMultiBits)]
pub fn derive_ls_multi_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_multi_common);
}
fn impl_ls_multi_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsMultiBits for #name {
            #[inline(always)]
            fn s(&self) -> bool {
                (self.0 & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn reglist(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_1111_1111_1111_1111)
            }
        }
    }
}


#[proc_macro_derive(LsBits)]
pub fn derive_ls_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_common);
}
fn impl_ls_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsBits for #name {
            #[inline(always)]
            fn p(&self) -> bool {
                (self.0 & 0b0000_0001_0000_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn u(&self) -> bool {
                (self.0 & 0b0000_0000_1000_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn b(&self) -> bool {
                (self.0 & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn w(&self) -> bool {
                (self.0 & 0b0000_0000_0010_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn l(&self) -> bool {
                (self.0 & 0b0000_0000_0001_0000_0000_0000_0000_0000) != 0
            }
        }
    }
}

#[proc_macro_derive(ImmBits)]
pub fn derive_imm_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_imm_common);
}
fn impl_imm_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ImmBits for #name {
            #[inline(always)]
            fn imm4(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0000_1111)
            }
            #[inline(always)]
            fn imm8(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_1111_1111)
            }
            #[inline(always)]
            fn imm12(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_1111_1111_1111)
            }

            #[inline(always)]
            fn imm12_hi(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_1111_1111_1111_0000_0000) >> 8
            }

            #[inline(always)]
            fn imm24(&self) -> u32 {
                (self.0 & 0b0000_0000_1111_1111_1111_1111_1111_1111)
            }
           #[inline(always)]
            fn off_hi(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
            }
            fn off_lo(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0000_1111)
            }
        }
    }
}


#[proc_macro_derive(BranchBits)]
pub fn derive_branch_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_branch_common);
}
fn impl_branch_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl BranchBits for #name {
            #[inline(always)]
            fn link(&self) -> bool {
                (self.0 & 0b0000_0001_0000_0000_0000_0000_0000_0000) != 0
            }
        }
    }
}


#[proc_macro_derive(RotBits)]
pub fn derive_rot_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_rot_common);
}
fn impl_rot_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl RotBits for #name {
            #[inline(always)]
            fn rot_imm(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
            }
        }
    }
}


#[proc_macro_derive(ShiftBits)]
pub fn derive_shift_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_shift_common);
}
fn impl_shift_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ShiftBits for #name {
            #[inline(always)]
            fn shift_imm(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_1111_1000_0000) >> 7
            }
            #[inline(always)]
            fn shift(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0110_0000) >> 5
            }
        }
    }
}

#[proc_macro_derive(CoprocBits)]
pub fn derive_coproc_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_coproc_common);
}
fn impl_coproc_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl CoprocBits for #name {
            #[inline(always)]
            fn opcd1(&self) -> u32 {
                (self.0 & 0b0000_0000_1111_0000_0000_0000_0000_0000) >> 20
            }
            #[inline(always)]
            fn opcd1_rt(&self) -> u32 {
                (self.0 & 0b0000_0000_1110_0000_0000_0000_0000_0000) >> 21
            }
            #[inline(always)]
            fn cp_num(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
            }
            #[inline(always)]
            fn opcd2(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_1110_0000) >> 5
            }
            #[inline(always)]
            fn crn(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16
            }
            #[inline(always)]
            fn crd(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12
            }
            #[inline(always)]
            fn crm(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0000_1111)
            }
        }
    }
}


#[proc_macro_derive(RegBits)]
pub fn derive_reg_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_reg_common);
}
fn impl_reg_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl RegBits for #name {
            #[inline(always)]
            fn rn(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16
            }
            #[inline(always)]
            fn rd(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12
            }
            #[inline(always)]
            fn rm(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0000_1111)
            }
            #[inline(always)]
            fn rs(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_0000_1111_0000_0000) >> 8
            }
        }
    }
}


#[proc_macro_derive(SrBits)]
pub fn derive_sr_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_sr_common);
}
fn impl_sr_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl SrBits for #name {
            #[inline(always)]
            fn field_mask(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16
            }
            #[inline(always)]
            fn r(&self) -> bool {
                (self.0 & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
            }
        }
    }
}

#[proc_macro_derive(MultiplyBits)]
pub fn derive_multiply_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_multiply_common);
}
fn impl_multiply_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl MultiplyBits for #name {
            #[inline(always)]
            fn rd_hi(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_1111_0000_0000_0000_0000) >> 16
            }
            #[inline(always)]
            fn rd_lo(&self) -> u32 {
                (self.0 & 0b0000_0000_0000_0000_1111_0000_0000_0000) >> 12
            }
            #[inline(always)]
            fn a(&self) -> bool {
                (self.0 & 0b0000_0000_0010_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn un(&self) -> bool {
                (self.0 & 0b0000_0000_0100_0000_0000_0000_0000_0000) != 0
            }
            #[inline(always)]
            fn x(&self) -> bool {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0010_0000) != 0
            }
            #[inline(always)]
            fn y(&self) -> bool {
                (self.0 & 0b0000_0000_0000_0000_0000_0000_0100_0000) != 0
            }
        }
    }
}


/* 
 * Derive macros for traits representing bitfields on Thumb instructions
 */


#[proc_macro_derive(DpFmt1Bits)]
pub fn derive_dp_fmt1(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt1);
}
fn impl_dp_fmt1(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt1Bits for #name {
            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rn(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn rm(&self) -> u16 { (self.0 & 0b0000_0001_1100_1000) >> 6 }

            #[inline(always)]
            fn op1(&self) -> bool { (self.0 & 0b0000_0010_0000_1000) != 0 }
        }
    }
}

#[proc_macro_derive(DpFmt2Bits)]
pub fn derive_dp_fmt2(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt2);
}
fn impl_dp_fmt2(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt2Bits for #name {
            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rn(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn imm3(&self) -> u16 { (self.0 & 0b0000_0001_1100_1000) >> 6 }

            #[inline(always)]
            fn op2(&self) -> bool { (self.0 & 0b0000_0010_0000_1000) != 0 }
        }
    }
}

#[proc_macro_derive(DpFmt3Bits)]
pub fn derive_dp_fmt3(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt3);
}
fn impl_dp_fmt3(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt3Bits for #name {
            #[inline(always)]
            fn op3(&self) -> u16 { (self.0 & 0b0001_1000_0000_0000) >> 11 }

            #[inline(always)]
            fn rdrn(&self) -> u16 { (self.0 & 0b0000_0111_0000_0000) >> 8 }

            #[inline(always)]
            fn imm8(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }
        }
    }
}

#[proc_macro_derive(DpFmt4Bits)]
pub fn derive_dp_fmt4(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt4);
}
fn impl_dp_fmt4(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt4Bits for #name {
            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rm(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn shift_imm(&self) -> u16 { 
                (self.0 & 0b0000_0111_1100_0000) >> 6
            }
            #[inline(always)]
            fn op4(&self) -> u16 { (self.0 & 0b0001_1000_0000_0000) >> 11 }
        }
    }
}

#[proc_macro_derive(DpFmt5Bits)]
pub fn derive_dp_fmt5(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt5);
}
fn impl_dp_fmt5(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt5Bits for #name {
            #[inline(always)]
            fn rdrn(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rmrs(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn op5(&self) -> u16 { (self.0 & 0b0000_0011_1100_0000) >> 6 }
        }
    }
}

#[proc_macro_derive(DpFmt6Bits)]
pub fn derive_dp_fmt6(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt6);
}
fn impl_dp_fmt6(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt6Bits for #name {
            #[inline(always)]
            fn reg(&self) -> bool { (self.0 & 0b0000_1000_0000_0000) != 0 }

            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0111_0000_0000) >> 8 }

            #[inline(always)]
            fn imm8(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }
        }
    }
}

#[proc_macro_derive(DpFmt7Bits)]
pub fn derive_dp_fmt7(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt7);
}
fn impl_dp_fmt7(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt7Bits for #name {
            #[inline(always)]
            fn op6(&self) -> bool { (self.0 & 0b0000_0000_1000_0000) != 0 }

            #[inline(always)]
            fn imm7(&self) -> u16 { (self.0 & 0b0000_0000_0111_1111) }

        }
    }
}

#[proc_macro_derive(DpFmt8Bits)]
pub fn derive_dp_fmt8(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_dp_fmt8);
}
fn impl_dp_fmt8(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl DpFmt8Bits for #name {
            #[inline(always)]
            fn rdrn(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rm(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn h2(&self) -> bool {(self.0 & 0b0000_0000_0100_0000) != 0 }

            #[inline(always)]
            fn h1(&self) -> bool {(self.0 & 0b0000_0000_1000_0000) != 0 }

            #[inline(always)]
            fn opcd(&self) -> u16 { (self.0 & 0b0000_0011_0000_0000) >> 8 }
        }
    }
}


#[proc_macro_derive(LsRegFmt1Bits)]
pub fn derive_ls_reg_fmt1(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_reg_fmt1);
}
fn impl_ls_reg_fmt1(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsRegFmt1Bits for #name {
            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rn(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn off(&self) -> u16 {(self.0 & 0b0000_0111_1100_0000) >> 6 }

            #[inline(always)]
            fn opcd1(&self) -> u16 {(self.0 & 0b1111_1000_1000_0000) >> 11 }
        }
    }
}


#[proc_macro_derive(LsRegFmt2Bits)]
pub fn derive_ls_reg_fmt2(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_reg_fmt2);
}
fn impl_ls_reg_fmt2(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsRegFmt2Bits for #name {
            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0000_0000_0111) }

            #[inline(always)]
            fn rn(&self) -> u16 { (self.0 & 0b0000_0000_0011_1000) >> 3 }

            #[inline(always)]
            fn rm(&self) -> u16 { (self.0 & 0b0000_0001_1100_0000) >> 6 }

            #[inline(always)]
            fn opcd2(&self) -> u16 { (self.0 & 0b1111_1110_0000_0000) >> 9 }
        }
    }
}

#[proc_macro_derive(LsRegFmt3Bits)]
pub fn derive_ls_reg_fmt3(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_reg_fmt3);
}
fn impl_ls_reg_fmt3(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsRegFmt3Bits for #name {
            #[inline(always)]
            fn imm8(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }

            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0111_0000_0000) >> 8 }
        }
    }
}

#[proc_macro_derive(LsRegFmt4Bits)]
pub fn derive_ls_reg_fmt4(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_reg_fmt4);
}
fn impl_ls_reg_fmt4(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsRegFmt4Bits for #name {
            #[inline(always)]
            fn imm8(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }

            #[inline(always)]
            fn rd(&self) -> u16 { (self.0 & 0b0000_0111_0000_0000) >> 8 }

            #[inline(always)]
            fn l(&self) -> bool { (self.0 & 0b0000_1000_0000_0000) != 0 }
        }
    }
}



#[proc_macro_derive(LsMultiFmt1Bits)]
pub fn derive_ls_multi_fmt1(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_multi_fmt1);
}
fn impl_ls_multi_fmt1(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsMultiFmt1Bits for #name {
            #[inline(always)]
            fn reglist(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }

            #[inline(always)]
            fn rn(&self) -> u16 { (self.0 & 0b0000_0111_0000_0000) >> 8 }

            #[inline(always)]
            fn l(&self) -> bool { (self.0 & 0b0000_1000_0000_0000) != 0 }
        }
    }
}

#[proc_macro_derive(LsMultiFmt2Bits)]
pub fn derive_ls_multi_fmt2(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_ls_multi_fmt2);
}
fn impl_ls_multi_fmt2(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl LsMultiFmt2Bits for #name {
            #[inline(always)]
            fn reglist(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }

            #[inline(always)]
            fn r(&self) -> bool { (self.0 & 0b0000_0001_0000_0000) != 0 }

            #[inline(always)]
            fn l(&self) -> bool { (self.0 & 0b0000_1000_0000_0000) != 0 }

        }
    }
}


#[proc_macro_derive(ThumbExcepBits)]
pub fn derive_thumb_excep(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_thumb_excep);
}
fn impl_thumb_excep(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl ThumbExcepBits for #name {
            #[inline(always)]
            fn imm8(&self) -> u16 { (self.0 & 0b0000_0000_1111_1111) }
        }
    }
}


