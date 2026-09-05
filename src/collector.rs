use super::*;

struct Candidate<'src> {
  frame: Rc<RefCell<Frame<'src>>>,
  incoming: usize,
  reachable: bool,
  references: Vec<usize>,
}

#[derive(Default)]
pub(crate) struct Collector<'src> {
  candidates: Vec<Candidate<'src>>,
  indices: HashMap<*const RefCell<Frame<'src>>, usize>,
}

impl<'src> Collector<'src> {
  pub(crate) fn collect(environment: &Environment<'src>) {
    let mut collector = Self::default();

    collector.environment(environment);

    let mut index = 0;

    while index < collector.candidates.len() {
      let frame = collector.candidates[index].frame.clone();

      if let Ok(frame) = frame.try_borrow() {
        if let Some(parent) = &frame.parent {
          let reference = collector.environment(parent);
          collector.candidates[index].references.push(reference);
        }

        for symbol in frame.symbols.values() {
          if let Some(function) = &symbol.function {
            collector.function(index, function);
          }

          if let Some(value) = &symbol.value {
            collector.value(index, value);
          }
        }
      } else {
        collector.candidates[index].reachable = true;
      }

      index += 1;
    }

    let mut pending = collector
      .candidates
      .iter()
      .enumerate()
      .filter(|(_, candidate)| {
        candidate.reachable
          || Rc::strong_count(&candidate.frame) > candidate.incoming + 1
      })
      .map(|(index, _)| index)
      .collect::<Vec<_>>();

    while let Some(index) = pending.pop() {
      let candidate = &mut collector.candidates[index];

      if candidate.reachable {
        continue;
      }

      candidate.reachable = true;
      pending.extend(&candidate.references);
    }

    let garbage = collector
      .candidates
      .iter()
      .filter(|candidate| !candidate.reachable)
      .map(|candidate| std::mem::take(&mut *candidate.frame.borrow_mut()))
      .collect::<Vec<_>>();

    drop(garbage);
  }

  fn environment(&mut self, environment: &Environment<'src>) -> usize {
    let index = *self
      .indices
      .entry(Rc::as_ptr(&environment.frame))
      .or_insert_with(|| {
        let index = self.candidates.len();

        self.candidates.push(Candidate {
          frame: environment.frame.clone(),
          incoming: 0,
          reachable: false,
          references: Vec::new(),
        });

        index
      });

    self.candidates[index].incoming += 1;

    index
  }

  fn function(&mut self, index: usize, function: &Function<'src>) {
    if let Function::UserDefined { environment, .. } = function {
      let reference = self.environment(environment);
      self.candidates[index].references.push(reference);
    }
  }

  fn value(&mut self, index: usize, value: &Value<'src>) {
    match value {
      Value::Function(function) => self.function(index, function),
      Value::List(values) => {
        for value in values {
          self.value(index, value);
        }
      }
      Value::Boolean(_) | Value::Null | Value::Number(_) | Value::String(_) => {
      }
    }
  }
}
