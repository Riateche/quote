use crate::ToTokens;
use proc_macro2::extra::DelimSpan;
use proc_macro2::{Span, TokenStream};
use std::borrow::Cow;

// Not public API other than via the syn crate. Use syn_send::spanned::Spanned.
pub trait Spanned: private::Sealed {
    fn __span(&self) -> Cow<'_, Span>;
}

impl Spanned for Span {
    fn __span(&self) -> Cow<'_, Span> {
        Cow::Borrowed(self)
    }
}

impl<'a> Spanned for Cow<'a, Span> {
    fn __span(&self) -> Cow<'_, Span> {
        match self {
            Cow::Borrowed(value) => Cow::Borrowed(value),
            Cow::Owned(value) => Cow::Borrowed(value),
        }
    }
}

impl Spanned for DelimSpan {
    fn __span(&self) -> Cow<'_, Span> {
        Cow::Borrowed(self.join())
    }
}

impl<T: ?Sized + ToTokens> Spanned for T {
    fn __span(&self) -> Cow<'_, Span> {
        Cow::Owned(join_spans(self.into_token_stream()))
    }
}

fn join_spans(tokens: TokenStream) -> Span {
    let mut iter = tokens.into_iter();

    let Some(first) = iter.next() else {
        return Span::call_site();
    };

    iter.fold(None, |_prev, next| Some(next))
        .and_then(|last| first.span().join(last.span()))
        .unwrap_or_else(|| first.span().clone())
}

mod private {
    use crate::ToTokens;
    use proc_macro2::extra::DelimSpan;
    use proc_macro2::Span;
    use std::borrow::Cow;

    pub trait Sealed {}
    impl Sealed for Span {}
    impl<'a> Sealed for Cow<'a, Span> {}
    impl Sealed for DelimSpan {}
    impl<T: ?Sized + ToTokens> Sealed for T {}
}
