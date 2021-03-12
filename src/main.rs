// this is a single linked list
// ignore the double parts, they are incomplete and wrong
// implement this in a new project with Option<Rc<RefCell<Noodle<T>>>>
#[derive(Debug)]
struct Noodle<T> {
    next: Nood<T>,
    // prev: Nood<T>,
    doot: T
}

type Nood<T> = Option<Box<Noodle<T>>>;

#[derive(Debug)]
struct DustyPawsie<T> {
    head: Nood<T>,
    // tail: Nood<T>,
}

impl<T> DustyPawsie<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn poosh(&mut self, doot: T) {
        let head = Noodle {
            next:  self.head.take(),
            doot
        };

        self.head = Some(Box::new(head));
    }

    // fn poosh_tail(&mut self, doot: T) {
    //     let tail = Noodle {
    //         next: None,
    //         prev: self.tail.take(),
    //         doot
    //     };

    //     self.tail = Some(Box::new(tail))
    // }

   fn pluck(&mut self) -> Option<T>  {
        self.head.take().map(|nood| {
            self.head = nood.next;
            nood.doot
        })
    }

   // fn pluck_tail(&mut self) -> Option<T> {
   //     self.tail.take().map(|nood| {
   //         self.tail = nood.prev;
   //         nood.doot
   //     })
   // }

   fn ser(&self) -> Option<&T> {
       self.head.as_ref().map(|nood| &nood.doot)
   }

   fn iter(&self) -> Iter<'_, T> {
       Iter {
           next: self.head.as_ref()
       }
   }
}

impl<T> Drop for DustyPawsie<T> {
    fn drop(&mut self) {
        let mut head = self.head.take();

        while let Some(mut current) = head {
            head = current.next.take();
        }
    }
}

struct Iter<'a, T> {
    next: Option<&'a Box<Noodle<T>>>
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|nood| {
            self.next = nood.next.as_ref();
            &nood.doot
        })
    }
}
// iter_mut is a copy of iter with mut in it Kappa

fn main() {
    let mut slink = DustyPawsie::new();
    slink.poosh(42);
    slink.poosh(82);
    println!("{:?}\n", slink);

    println!("{:?}", slink.pluck());
    println!("{:?}\n", slink);

    println!("{:?}\n", slink.ser());

    slink.poosh(42);
    slink.poosh(84);
    let mut slink_iter = slink.iter();
    while let Some(noodle) = slink_iter.next(){
        println!("{:?}", noodle);
    }

    println!("\n{:?}", slink);
    drop(slink);
}

