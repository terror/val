use {
  criterion::{Criterion, black_box, criterion_group, criterion_main},
  val::{Environment, Evaluator},
};

fn bench_increment_value(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("increment_value");

  for &number in &[10_u32, 50, 100, 500] {
    let program =
      format!("a = 0.001; while (a < {number}) {{ a = a + 0.001 }}; a");

    let ast = val::parse(&program).unwrap();

    group.bench_function(format!("n = {number}"), |bencher| {
      bencher.iter(|| {
        black_box(Evaluator::from(Environment::default()).eval(&ast)).unwrap();
      })
    });
  }

  group.finish();
}

fn bench_prime_count(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("prime_count");

  for &number in &[5_000_u32, 10_000_u32] {
    let program = format!(
      r#"
      fn prime(n) {{
        if (n < 2) {{
          return false
        }}

        i = 2

        while (i * i <= n) {{
          if (n % i == 0) {{
            return false
          }}

          i = i + 1
        }}

        return true
      }}

      fn count(limit) {{
        count = 0

        i = 2

        while (i <= limit) {{
          if (prime(i)) {{
            count = count + 1
          }}

          i = i + 1
        }}

        return count
      }}

      count({number})
      "#
    );

    let ast = val::parse(&program).unwrap();

    group.bench_function(format!("n = {number}"), |bencher| {
      bencher.iter(|| {
        black_box(Evaluator::from(Environment::default()).eval(&ast)).unwrap();
      });
    });
  }

  group.finish();
}

fn bench_recursive_factorial(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("recursive_factorial");

  for &number in &[10_u32, 50, 100, 500] {
    let program = format!(
      "fn f(x) {{ if ( x <= 1) {{ return 1 }} else {{ return x * f(x - 1) }} }} f({number})"
    );

    let ast = val::parse(&program).unwrap();

    group.bench_function(format!("n = {number}"), |bencher| {
      bencher.iter(|| {
        black_box(Evaluator::from(Environment::default()).eval(&ast)).unwrap();
      })
    });
  }

  group.finish();
}

criterion_group!(
  benches,
  bench_increment_value,
  bench_prime_count,
  bench_recursive_factorial
);

criterion_main!(benches);
