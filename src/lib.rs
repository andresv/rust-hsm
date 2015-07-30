#[macro_use]
extern crate log;

#[macro_use]
mod macros;

#[cfg(test)]
mod tests;


macro_rules! paste_action_fmt_fn {
    () => {
        fn fmt(&self, f:&mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
            match *self {
                Action::Ignore         => ::std::fmt::Display::fmt("Ignore", f),
                Action::Transition(..) => ::std::fmt::Display::fmt("Transition", f),
            };
            Ok(())
        }
    }
}
enum Action<'a, UsrEvt> {
    Ignore,
    Transition(&'a State<'a, UsrEvt>)
}
impl<'a, UsrEvt> ::std::fmt::Display for Action<'a, UsrEvt> {
    paste_action_fmt_fn!();
}
impl<'a, UsrEvt> ::std::fmt::Debug for Action<'a, UsrEvt> {
    paste_action_fmt_fn!();
}


#[derive(Debug)]
enum Event<UsrEvt> {
    Enter,
    User(UsrEvt),
    Exit
}

trait State<'a, UsrEvt> {
    fn handle_event(&'a mut self, evt: Event<UsrEvt>) -> Action<'a, UsrEvt>;
}

trait StateLookup<StEnum, UsrEvt> {
    fn lookup(&mut self, typ: &StEnum) -> &mut State<UsrEvt>;
}

trait Initializer {
    fn new() -> Self;
}


struct StateMachine<UsrEvt, UsrStEnum, UsrSt> {
    current  : UsrStEnum,
    states   : UsrSt,
    _phantom : ::std::marker::PhantomData<UsrEvt>
}
impl<UsrEvt, UsrStEnum, UsrSt> StateMachine<UsrEvt, UsrStEnum, UsrSt>
    where UsrSt     : Initializer + StateLookup<UsrStEnum, UsrEvt>,
          UsrEvt    : ::std::fmt::Debug,
          UsrStEnum : ::std::fmt::Debug  {
    fn new(initial: UsrStEnum) -> Self {
        StateMachine {
            current  : initial,
            states   : UsrSt::new(),
            _phantom : ::std::marker::PhantomData
        }
    }

    fn input(&mut self, evt: Event<UsrEvt>) {
        info!("input:  {:?}", evt);
        info!("state:  {:?}", self.current);
        let cur_st = self.states.lookup(&self.current);
        let action = cur_st.handle_event(evt);
        info!("action: {:?}", action)

    }
}