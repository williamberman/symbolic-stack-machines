use symbolic_stack_machines_contrib::instructions::misc::PUSH;
use symbolic_stack_machines_core::{
    instructions::{EnvExtension, EnvExtensionRecord},
    machine::{
        inner_interpreter::ConcreteInnerInterpreter,
        outer_interpreter::{ConcreteOuterInterpreter, OuterInterpreter},
        r#abstract::AbstractMachine,
    },
    memory::{Mem, WriteableMem},
    stack::{BaseStack, Stack},
};

#[derive(Clone)]
pub struct DummyExtEnv {}

impl EnvExtension for DummyExtEnv {
    type InnerValue = Option<()>;

    type ErrorType = Option<()>;

    type IndexType = Option<()>;

    type DiffRecordType = DummyExtEnvRecord;

    fn write<V: Into<Self::InnerValue>>(&self, _v: V) -> Result<Self, Self::ErrorType>
    where
        Self: Sized,
    {
        todo!()
    }

    fn read<I: Into<Self::IndexType>>(&self, _idx: I) -> Result<Self::InnerValue, Self::ErrorType> {
        todo!()
    }
}

pub struct DummyExtEnvRecord {}

impl EnvExtensionRecord for DummyExtEnvRecord {
    fn apply<E: EnvExtension>(&self, env: E) -> Result<E, E::ErrorType> {
        Ok(env)
    }
}

#[derive(Clone)]
pub struct DummyMem {}

impl Mem for DummyMem {
    type MemVal = ();
    type Index = ();
}

impl WriteableMem for DummyMem {
    fn write(
        &self,
        idx: Self::Index,
        val: Self::MemVal,
    ) -> symbolic_stack_machines_core::memory::MemoryResult<Self> {
        todo!()
    }
}

#[test]
fn test_abstract_machine() {
    let pgm = vec![PUSH(1)];
    let custom_env = DummyExtEnv {};
    let pc = Some(0);
    let mem = DummyMem {};
    let stack = BaseStack::<i32>::init();
    let machine = AbstractMachine {
        stack,
        mem,
        custom_env,
        pc,
        pgm: &pgm,
    };
    let inner_interpreter = Box::new(ConcreteInnerInterpreter {});
    let outer_interpreter = ConcreteOuterInterpreter { inner_interpreter };

    let res: Option<i32> = outer_interpreter.run(machine).unwrap().stack.peek(0);

    assert_eq!(res, Some(1))
}
