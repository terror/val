use super::*;

macro_rules! builtin {
  (
    $builtin:ident {
      name: $name:literal,
      $(aliases: [$($alias:literal),* $(,)?],)?
      arity: $arity:expr,
      call($payload:ident) $body:block
      $(,)?
    }
  ) => {
    struct $builtin;

    impl $builtin {
      #[allow(clippy::unnecessary_wraps)]
      fn call<'src>(
        $payload: &BuiltinFunctionPayload<'src>,
      ) -> Result<Value<'src>, Error> $body
    }

    impl Builtin for $builtin {
      $(
        fn aliases(&self) -> &'static [&'static str] {
          &[$($alias),*]
        }
      )?

      fn kind(&self) -> &'static str {
        "function"
      }

      fn name(&self) -> &'static str {
        $name
      }

      fn value<'src>(&self, _: Config) -> Value<'src> {
        Value::Function(Function::Builtin {
          arity: $arity,
          function: Self::call,
          name: self.name(),
        })
      }
    }

    inventory::submit!(&$builtin as &dyn Builtin);
  };
  (
    $builtin:ident {
      name: $name:literal,
      $(aliases: [$($alias:literal),* $(,)?],)?
      constant($config:ident) $body:block
      $(,)?
    }
  ) => {
    struct $builtin;

    impl $builtin {
      fn constant($config: Config) -> Number $body
    }

    impl Builtin for $builtin {
      $(
        fn aliases(&self) -> &'static [&'static str] {
          &[$($alias),*]
        }
      )?

      fn kind(&self) -> &'static str {
        "constant"
      }

      fn name(&self) -> &'static str {
        $name
      }

      fn value<'src>(&self, config: Config) -> Value<'src> {
        Value::Number(Self::constant(config))
      }
    }

    inventory::submit!(&$builtin as &dyn Builtin);
  };
}

inventory::collect!(&'static dyn Builtin);

mod abs;
mod acos;
mod acot;
mod acsc;
mod append;
mod arc;
mod asec;
mod asin;
mod r#bool;
mod ceil;
mod constant_e;
mod constant_phi;
mod constant_pi;
mod constant_tau;
mod cos;
mod cosh;
mod cot;
mod csc;
mod e;
mod exit;
mod float;
mod floor;
mod gcd;
mod input;
mod int;
mod join;
mod lcm;
mod len;
mod list;
mod ln;
mod log10;
mod log2;
mod print;
mod println;
mod range;
mod sec;
mod sin;
mod sinh;
mod split;
mod sqrt;
mod sum;
mod tan;
mod tanh;

pub trait Builtin: Sync {
  fn aliases(&self) -> &'static [&'static str] {
    &[]
  }

  fn kind(&self) -> &'static str;

  fn name(&self) -> &'static str;

  fn value<'src>(&self, config: Config) -> Value<'src>;
}
