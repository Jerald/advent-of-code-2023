pub mod prelude {
    pub trait DebugInspect: Iterator {
        fn debug_inspect<F>(self, f: F) -> std::iter::Inspect<Self, impl FnMut(&Self::Item)>
            where
                Self: Sized,
                F: FnMut(&Self::Item)
        {
            #[cfg(debug_assertions)]
            let out = self.inspect(f);
    
            #[cfg(not(debug_assertions))]
            let out = self.inspect(|_|{});
    
            out
        }
    }
    
    impl<I: Iterator> DebugInspect for I {}
}