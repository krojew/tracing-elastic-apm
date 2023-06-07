

pub struct ApmSubscriber<C> {
   
    get_context: WithContext,
    _collector: PhantomData<fn(C)>,
}