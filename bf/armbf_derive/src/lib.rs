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


#[proc_macro_derive(InstBits)]
pub fn derive_inst_common(input: TokenStream) -> TokenStream {
    return get_tokenstream!(input, impl_inst_common);
}
fn impl_inst_common(ast: &syn::DeriveInput) -> quote::Tokens {
    let name = &ast.ident;
    quote! {
        impl InstBits for #name {
            #[inline(always)]
            fn cond(&self) -> u32 { get_cond!(self.0) }
            #[inline(always)]
            fn group(&self) -> u32 { get_group!(self.0) }
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
            fn opcd(&self) -> u32 { get_opcd!(self.0) }
            #[inline(always)]
            fn s(&self) -> bool { get_s!(self.0) }
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
            fn s(&self) -> bool { get_s_multi!(self.0) }
            #[inline(always)]
            fn reglist(&self) -> u32 { get_reglist!(self.0) }
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
            fn p(&self) -> bool { get_p!(self.0) }
            #[inline(always)]
            fn u(&self) -> bool { get_u!(self.0) }
            #[inline(always)]
            fn b(&self) -> bool { get_b!(self.0) }
            #[inline(always)]
            fn w(&self) -> bool { get_w!(self.0) }
            #[inline(always)]
            fn l(&self) -> bool { get_l!(self.0) }
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
            fn imm4(&self) -> u32 { get_imm4!(self.0) }
            #[inline(always)]
            fn imm8(&self) -> u32 { get_imm8!(self.0) }
            #[inline(always)]
            fn imm12(&self) -> u32 { get_imm12!(self.0) }

            #[inline(always)]
            fn imm12_hi(&self) -> u32 { get_imm12_hi!(self.0) }

            #[inline(always)]
            fn imm24(&self) -> u32 { get_imm24!(self.0) }

           #[inline(always)]
            fn off_hi(&self) -> u32 { get_off_hi!(self.0) }
            fn off_lo(&self) -> u32 { get_off_lo!(self.0) }


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
            fn link(&self) -> bool { get_link!(self.0) }
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
            fn rot_imm(&self) -> u32 { get_rot_imm!(self.0) }
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
            fn shift_imm(&self) -> u32 { get_shift_imm!(self.0) }
            #[inline(always)]
            fn shift(&self) -> u32 { get_shift!(self.0) }
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
            fn opcd1(&self) -> u32 { get_opcd1!(self.0) }
            #[inline(always)]
            fn opcd1_rt(&self) -> u32 { get_opcd1_rt!(self.0) }
            #[inline(always)]
            fn cp_num(&self) -> u32 { get_cp_num!(self.0) }
            #[inline(always)]
            fn opcd2(&self) -> u32 { get_opcd2!(self.0) }
            #[inline(always)]
            fn crn(&self) -> u32 { get_crn!(self.0) }
            #[inline(always)]
            fn crd(&self) -> u32 { get_crd!(self.0) }
            #[inline(always)]
            fn crm(&self) -> u32 { get_crm!(self.0) }
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
            fn rn(&self) -> u32 { get_rn!(self.0) }
            #[inline(always)]
            fn rd(&self) -> u32 { get_rd!(self.0) }
            #[inline(always)]
            fn rm(&self) -> u32 { get_rm!(self.0) }
            #[inline(always)]
            fn rs(&self) -> u32 { get_rs!(self.0) }
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
            fn field_mask(&self) -> u32 { get_field_mask!(self.0) }
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
            fn rd_hi(&self) -> u32 { get_rd_hi!(self.0) }
            #[inline(always)]
            fn rd_lo(&self) -> u32 { get_rd_lo!(self.0) }
            #[inline(always)]
            fn a(&self) -> bool { get_a!(self.0) }
            #[inline(always)]
            fn un(&self) -> bool { get_un!(self.0) }
            #[inline(always)]
            fn x(&self) -> bool { get_x!(self.0) }
            #[inline(always)]
            fn y(&self) -> bool { get_y!(self.0) }
        }
    }
}




#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}


