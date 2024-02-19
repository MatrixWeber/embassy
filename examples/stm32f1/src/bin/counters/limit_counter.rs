pub struct LimitCounter
{
    value: usize,
    limit: usize,
}

pub trait Reset
{
    fn reset(&mut self);
}

pub trait Count
{
    fn count(&mut self);
}

pub trait Show
{
    fn get_counter(&self) -> usize;
}

impl LimitCounter {
    pub fn new(value: usize, limit: usize) -> LimitCounter {
        LimitCounter { value, limit }
    }

}

impl Show for LimitCounter {
    fn get_counter(&self) -> usize { self.value }
}

impl Count for LimitCounter {
    fn count(&mut self)
    {
        if self.value < self.limit
        {
            self.value += 1;
        } else {
            self.reset();
        }
    }
}

impl Reset for LimitCounter {
    fn reset(&mut self)
    {
        self.value = 0;
    }
}
