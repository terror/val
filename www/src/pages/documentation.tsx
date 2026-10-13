import { DocumentationMarkdown } from '@/components/documentation-markdown';
import { Header } from '@/components/header';
import { useTheme } from '@/hooks/use-theme';
import { documentation } from '@/lib/documentation';
import { ChevronRight } from 'lucide-react';
import { useEffect } from 'react';
import { Link, useLocation } from 'react-router';

const DocumentationNavigation = () => {
  const { hash } = useLocation();

  return (
    <nav aria-label='Documentation navigation'>
      <ul className='space-y-6 text-sm'>
        {documentation.map(({ id, title, headings }) => (
          <li key={id}>
            <details open>
              <summary className='flex cursor-pointer list-none items-center gap-1 rounded-sm py-1 focus-visible:outline-2 focus-visible:outline-offset-2 [&::-webkit-details-marker]:hidden [[open]>&>svg]:rotate-90'>
                <ChevronRight aria-hidden='true' className='h-4 w-4 shrink-0' />
                <Link
                  to={`#${id}`}
                  aria-current={hash === `#${id}` ? 'location' : undefined}
                  className='min-w-0 rounded-sm font-semibold break-words hover:underline hover:underline-offset-4 focus-visible:outline-2 focus-visible:outline-offset-2'
                >
                  {title}
                </Link>
              </summary>
              <ul className='mt-2 ml-2 space-y-1 border-l pl-3'>
                {headings.map(({ id, title }) => (
                  <li key={id}>
                    <Link
                      to={`#${id}`}
                      aria-current={hash === `#${id}` ? 'location' : undefined}
                      className='text-muted-foreground hover:text-foreground aria-[current=location]:text-foreground block rounded-sm py-1 break-words focus-visible:outline-2 focus-visible:outline-offset-2 aria-[current=location]:font-medium'
                    >
                      {title}
                    </Link>
                  </li>
                ))}
              </ul>
            </details>
          </li>
        ))}
      </ul>
    </nav>
  );
};

const Documentation = () => {
  const theme = useTheme();
  const { hash, key } = useLocation();

  useEffect(() => {
    document.title = 'Documentation - val';
  }, []);

  useEffect(() => {
    if (hash) {
      document.getElementById(hash.slice(1))?.scrollIntoView();
    } else {
      window.scrollTo(0, 0);
    }
  }, [hash, key]);

  return (
    <div className='min-h-dvh'>
      <Header darkMode={theme.darkMode} onToggleTheme={theme.toggleTheme} />

      <div className='mx-auto max-w-6xl px-6 py-12 lg:grid lg:grid-cols-[14rem_minmax(0,1fr)] lg:gap-12 lg:py-16'>
        <aside className='hidden lg:block'>
          <div className='sticky top-6 max-h-[calc(100dvh-3rem)] overflow-y-auto pr-4'>
            <p className='mb-3 text-sm font-semibold'>Documentation</p>
            <DocumentationNavigation />
          </div>
        </aside>

        <main className='min-w-0 leading-[1.75]'>
          <h1 className='font-mono text-3xl font-semibold tracking-tight'>
            Documentation
          </h1>
          <p className='text-muted-foreground mt-4 text-lg'>
            A guide to calculating and writing programs with val.
          </p>

          <details className='my-8 rounded-md border p-4 lg:hidden'>
            <summary className='cursor-pointer font-medium'>
              On this page
            </summary>
            <div className='mt-4 max-h-96 overflow-y-auto'>
              <DocumentationNavigation />
            </div>
          </details>

          <div className='mt-12'>
            {documentation.map((section) => (
              <section
                key={section.id}
                aria-labelledby={section.id}
                className='mt-12 border-t pt-12 first:mt-0 first:border-t-0 first:pt-0'
              >
                <DocumentationMarkdown section={section} />
              </section>
            ))}
          </div>
        </main>
      </div>
    </div>
  );
};

export default Documentation;
