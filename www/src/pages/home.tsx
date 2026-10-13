import { Header } from '@/components/header';
import { Button } from '@/components/ui/button';
import { useTheme } from '@/hooks/use-theme';
import { useEffect } from 'react';
import { Link } from 'react-router';

import screenshot from '../../../screenshot.png';

const Home = () => {
  const theme = useTheme();

  useEffect(() => {
    document.title = 'val';
    window.scrollTo(0, 0);
  }, []);

  return (
    <div className='min-h-dvh'>
      <Header darkMode={theme.darkMode} onToggleTheme={theme.toggleTheme} />

      <main className='mx-auto max-w-3xl px-6 py-12 leading-[1.75] sm:py-16'>
        <h1 className='font-mono text-3xl font-semibold tracking-tight'>val</h1>
        <p className='text-muted-foreground mt-4 text-lg'>
          An arbitrary precision calculator language.
        </p>
        <p className='text-muted-foreground mt-4'>
          Work with exact fractions and decimals, write functions, and explore
          calculations in an interactive REPL. Use val for a quick expression in
          your terminal or save a program to run again later.
        </p>
        <div className='mt-6 flex flex-wrap gap-3'>
          <Button asChild>
            <Link to='/documentation#installation'>Get started</Link>
          </Button>
          <Button asChild variant='outline'>
            <Link to='/playground'>Try the playground</Link>
          </Button>
        </div>
        <img
          src={screenshot}
          alt='The val REPL showing arithmetic, mathematical functions, syntax highlighting, and error diagnostics'
          width={3334}
          height={2166}
          className='mt-8 h-auto w-full rounded-md'
        />

        <section aria-labelledby='installation' className='mt-12'>
          <h2
            id='installation'
            className='mb-4 text-xl font-semibold tracking-tight'
          >
            Installation
          </h2>
          <p className='text-muted-foreground'>Install with Cargo:</p>
          <pre className='bg-muted my-4 overflow-x-auto rounded-md border px-4 py-3 text-sm'>
            <code>cargo install val</code>
          </pre>
          <p className='text-muted-foreground'>Or with Homebrew:</p>
          <pre className='bg-muted my-4 overflow-x-auto rounded-md border px-4 py-3 text-sm'>
            <code>brew install terror/tap/val</code>
          </pre>
          <p className='text-muted-foreground'>
            <a
              href='https://github.com/terror/val/releases'
              className='text-foreground underline decoration-current/30 underline-offset-4 hover:decoration-current'
            >
              Pre-built binaries
            </a>{' '}
            are available for Linux, macOS, and Windows. See the{' '}
            <Link
              to='/documentation#installation'
              className='text-foreground underline decoration-current/30 underline-offset-4 hover:decoration-current'
            >
              installation guide
            </Link>{' '}
            to get started.
          </p>
        </section>

        <section aria-labelledby='usage' className='mt-12'>
          <h2 id='usage' className='mb-4 text-xl font-semibold tracking-tight'>
            Usage
          </h2>
          <p className='text-muted-foreground'>
            Run <code className='text-foreground text-sm'>val</code> to start
            the REPL, or evaluate an expression from your shell:
          </p>
          <pre className='bg-muted my-4 overflow-x-auto rounded-md border px-4 py-3 text-sm'>
            <code>{`$ val -e '0.1 + 0.2'
0.3

$ val -e '(1 / 3) * 3'
1`}</code>
          </pre>
          <p className='text-muted-foreground'>
            Calculations stay exact where possible. For approximate math, choose
            your precision, rounding mode, and displayed digits. Functions,
            loops, lists, and built-in math operations let you turn a
            calculation into a program.
          </p>
          <pre className='bg-muted my-4 overflow-x-auto rounded-md border px-4 py-3 text-sm'>
            <code>{`fn factorial(n) {
  if (n <= 1) {
    return 1
  }

  return n * factorial(n - 1)
}

println(factorial(5))`}</code>
          </pre>
          <p className='text-muted-foreground'>
            Save this as{' '}
            <code className='text-foreground text-sm'>factorial.val</code> and
            run{' '}
            <code className='text-foreground text-sm'>val factorial.val</code>{' '}
            to print <code className='text-foreground text-sm'>120</code>.
          </p>
        </section>

        <section aria-labelledby='resources' className='mt-12'>
          <h2
            id='resources'
            className='mb-4 text-xl font-semibold tracking-tight'
          >
            Resources
          </h2>
          <ul className='text-muted-foreground [&_a]:text-foreground list-disc space-y-2 pl-5 [&_a]:underline [&_a]:decoration-current/30 [&_a]:underline-offset-4 [&_a:hover]:decoration-current'>
            <li>
              <Link to='/documentation#usage'>Command line</Link>: evaluate
              expressions, run files, and load programs into the REPL.
            </li>
            <li>
              <Link to='/documentation#language'>Language</Link>: learn about
              values, control flow, and functions.
            </li>
            <li>
              <Link to='/documentation#numbers'>Numbers and precision</Link>:
              understand exact arithmetic and configure approximate math.
            </li>
            <li>
              <Link to='/documentation#builtins'>Built-ins</Link>: browse the
              available functions and constants.
            </li>
            <li>
              <Link to='/playground'>Playground</Link>: run val and inspect its
              syntax tree in your browser.
            </li>
          </ul>
        </section>
      </main>
    </div>
  );
};

export default Home;
