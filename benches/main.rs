use {
  criterion::{Criterion, criterion_group, criterion_main},
  rug::{Integer, Rational},
  std::hint::black_box,
  val::{Config, Environment, Evaluator, Number},
};

fn bench_decimal_display(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("decimal_display");

  for &(factor, exponent) in &[(2, 100), (2, 1_000), (5, 100), (5, 1_000)] {
    let mut denominator = Integer::from(Integer::u_pow_u(factor, exponent));
    denominator *= 3;

    let number = Number::Exact(Rational::from((1, denominator)));

    group.bench_function(format!("3 * {factor}^{exponent}"), |bencher| {
      bencher.iter(|| black_box(number.display(Config::default())));
    });
  }

  group.finish();
}

fn bench_increment_value(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("increment_value");

  for &number in &[10_u32, 50, 100, 500] {
    let program =
      format!("a = 0.001; while (a < {number}) {{ a = a + 0.001 }}; a");

    let ast = val::parse(&program).unwrap();

    group.bench_function(format!("n = {number}"), |bencher| {
      bencher.iter(|| {
        black_box(Evaluator::from(Environment::default()).evaluate(&ast))
          .unwrap();
      });
    });
  }

  group.finish();
}

fn bench_prime_count(criterion: &mut Criterion) {
  let mut group = criterion.benchmark_group("prime_count");

  for &number in &[5_000_u32, 10_000_u32] {
    let program = format!(
      r"
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
      "
    );

    let ast = val::parse(&program).unwrap();

    group.bench_function(format!("n = {number}"), |bencher| {
      bencher.iter(|| {
        black_box(Evaluator::from(Environment::default()).evaluate(&ast))
          .unwrap();
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
        black_box(Evaluator::from(Environment::default()).evaluate(&ast))
          .unwrap();
      });
    });
  }

  group.finish();
}

criterion_group!(
  benches,
  bench_decimal_display,
  bench_increment_value,
  bench_prime_count,
  bench_recursive_factorial
);

criterion_main!(benches);
