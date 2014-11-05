/* INFO **
** INFO */

/* Use modules of the std crate */
use std::fmt;

/* Use cassowary modules */
use super::variable;


/*----------------------------------------------------------------------------*/
pub struct Point<T: Num+Copy+fmt::Show>
{
    x: variable::GenericVariable<T>,
    y: variable::GenericVariable<T>,
    // name  : String,
}

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl<T: Num+Copy+fmt::Show> Point<T>
{
    pub fn new(x: T,
               y: T) -> Point<T>
    {
        Point
        {
            x: variable::Variable::new(None, Some(x)),
            y: variable::Variable::new(None, Some(y)),
        }
    }

    pub fn set(&self,
               x: T,
               y: T) -> &Point<T>
    {
        self.x.set(Some(x));
        self.y.set(Some(y));
        self
    }

}

/*- - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - */
impl<T: Num+Copy+fmt::Show> fmt::Show for Point<T>
{
    fn fmt(&self,
           f: &mut fmt::Formatter) -> fmt::Result
    {
        write!(f, "Pt({}, {})", self.x, self.y)
    }
}
