/* INFO **
** INFO */

/* Use modules of the std crate */
use std::fmt;
use std::num::{Num, zero};
use std::sync::atomic::{AtomicUint, SeqCst, INIT_ATOMIC_UINT};

/* Module level constants */
static ID_FACTORY: AtomicUint = INIT_ATOMIC_UINT;


/*----------------------------------------------------------------------------*/
/* Base Trait */
pub trait Variable<T: Num+Copy+fmt::Show>
{
    fn new(name  : Option<&str>,
           value : Option<T>) -> Self;

    fn set(&self,
           value: Option<T>) -> &Self;
}



/*----------------------------------------------------------------------------*/
/* Helper functions */
#[inline]
fn get_var_name(name: Option<&str>) -> String
{
    match name
    {
        Some(text) => text.to_string(),
        None       => format!("var{}", ID_FACTORY.fetch_add(1u, SeqCst)),
    }
}

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
#[inline]
fn get_num_value<T: Num+Copy+fmt::Show>(value: Option<T>) -> T
{
    match value
    {
        Some(number) => number,
        None         => zero(),
    }
}



/*----------------------------------------------------------------------------*/
/* Base Type */
pub struct GenericVariable<T: Num+Copy+fmt::Show>
{
    name  : String,
    value : T,
}
/*
pub struct DummyVariable<T>
{
    name  : &String,
    value : T,
}

pub struct ObjectiveVariable<T>
{
    name  : &String,
    value : T,
}

pub struct SlackVariable<T>
{
    name  : &String,
    value : T,
}
*/

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl<T: Num+Copy+fmt::Show> Variable<T> for GenericVariable<T>
{
    fn new(name  : Option<&str>,
           value : Option<T>) -> GenericVariable<T>
    {
        GenericVariable
        {
            name  : get_var_name(name),
            value : get_num_value(value),
        }
    }

    /*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
    fn set(&self,
           value: Option<T>) -> &GenericVariable<T>
    {
        self.value = match value
        {
            Some(num) => num,
            None      => zero(),
        };
        self
    }
}

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl<T: Num+Copy+fmt::Show> fmt::Show for GenericVariable<T>
{
    fn fmt(&self,
           f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "[{}:{}]", self.name, self.value)
    }
}
