use std::borrow::{Borrow, Cow};
pub struct Owned<T>(pub T);

pub trait MaybeOwned<T: ?Sized + ToOwned> {
    fn into_owned(self) -> T::Owned;
    fn borrow(&self) -> &T;
}

impl<T: ToOwned + ?Sized> MaybeOwned<T> for &T {
    fn into_owned(self) -> T::Owned {
        self.to_owned()
    }
    fn borrow(&self) -> &T {
        self
    }
}

impl<B: ?Sized + ToOwned> MaybeOwned<B> for Owned<B::Owned> {
    fn into_owned(self) -> B::Owned {
        self.0
    }
    fn borrow(&self) -> &B {
        self.0.borrow()
    }
}

impl<'a, T: ToOwned + ?Sized> MaybeOwned<T> for Cow<'a, T> {
    fn into_owned(self) -> <T as ToOwned>::Owned {
        self.into_owned()
    }
    fn borrow(&self) -> &T {
        Borrow::borrow(self)
    }
}

#[test]
fn test() {
    struct Noisy<T: ?Sized>(pub T);

    impl ToOwned for Noisy<str> {
        type Owned = Noisy<String>;

        fn to_owned(&self) -> Self::Owned {
            println!("CLONE {}", &self.0);
            Noisy(self.0.to_owned())
        }
    }

    impl Borrow<Noisy<str>> for Noisy<String> {
        fn borrow(&self) -> &Noisy<str> {
            println!("BORROW {}", &self.0);
            unsafe { &*(self.0.as_str() as *const str as *const Noisy<str>) }
        }
    }

    fn foo<S: MaybeOwned<Noisy<str>>>(s: S, requests: &mut Vec<String>) {
        let _ = s.borrow().0.len();
        requests.push(s.into_owned().0);
    }

    let mut reqs = Vec::new();
    foo(Noisy("borrow".to_string()).borrow(), &mut reqs);
    foo(Owned(Noisy("owned".to_string())), &mut reqs);
    foo(
        Cow::Borrowed(Noisy("cow borrow".to_string()).borrow()),
        &mut reqs,
    );
    foo(Cow::Owned(Noisy("cow owned".to_string())), &mut reqs);
}
