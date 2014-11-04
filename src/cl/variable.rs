/* INFO **
** INFO */

/* Use modules of the std crate */
use std::fmt;
use std::sync::atomic::{AtomicUint, SeqCst, INIT_ATOMIC_UINT};

/* Module level constants */
static ID_FACTORY: AtomicUint = INIT_ATOMIC_UINT;


/*----------------------------------------------------------------------------*/
trait AbstractVariable
{
    fn new(name: &str,
           prefix: &str) -> Self;
}



/*----------------------------------------------------------------------------*/
pub struct Variable
{
    id: uint,
    name: String,
}


/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl AbstractVariable for Variable
{
    fn new(name: &str,
           prefix: &str) -> Variable
    {
        /* Get new id */
        let id = ID_FACTORY.fetch_add(1u, SeqCst);
        /* Create and return a new object */
        Variable
        {
            id: id,
            name: format!("v{}", id),
        }
    }
}


/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl fmt::Show for Variable
{
    fn fmt(&self,
           f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "[{}]", self.name)
    }
}


/*
/*----------------------------------------------------------------------------*/
struct DummyVariable
{
    hash_code: uint,
    _name: String,
}


/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl AbstractVariable for DummyVariable
{

}


/*----------------------------------------------------------------------------*/
struct ObjectiveVariable
{
    hash_code: uint,
    _name: String,
}


/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl AbstractVariable for ObjectiveVariable
{

}



/*----------------------------------------------------------------------------*/
struct SlackVariable
{
    hash_code: uint,
    _name: String,
}


/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl AbstractVariable for SlackVariable
{

}
*/