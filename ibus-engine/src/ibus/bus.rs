//! Abstraction of the whole ibus
use std::error::Error;
use std::marker::PhantomData;

use log::debug;
use zbus::Connection;

use super::{IBusEngine, IBusFactory};

use super::factory::{Factory, register_factory};
use super::init::{connect_ibus, request_name};

/// Abstract for the whole ibus
#[derive(Debug, Clone)]
pub struct IBus<T: IBusEngine, U: IBusFactory<T>> {
    _t: PhantomData<T>,
    _u: PhantomData<U>,

    c: Connection,
}

impl<T: IBusEngine + 'static, U: IBusFactory<T> + 'static> IBus<T, U> {
    /// connect to ibus and init
    ///
    /// only support 1 engine now.
    pub async fn new(addr: String, factory: U, name: String) -> Result<Self, Box<dyn Error>> {
        let c = connect_ibus(addr).await?;
        debug!("connected to ibus");

        {
            let f = Factory::new(c.clone(), factory);
            register_factory(&c, f).await?;
        }
        debug!("factory registered");

        request_name(&c, name.clone()).await?;
        debug!("requested name: {}", name);

        Ok(Self {
            _t: PhantomData,
            _u: PhantomData,
            c,
        })
    }

    /// get D-Bus connection
    pub fn conn(&self) -> Connection {
        self.c.clone()
    }
}
