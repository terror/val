import { documentation } from '@/lib/documentation';
import type { ComponentProps } from 'react';
import Markdown from 'react-markdown';
import { Link } from 'react-router';
import remarkGfm from 'remark-gfm';

const DocumentationLink = ({ href, children }: ComponentProps<'a'>) => {
  const section = documentation.find(({ id }) => href === `${id}.md`);
  const to = section ? `/documentation#${section.id}` : href;

  return to?.startsWith('/') || to?.startsWith('#') ? (
    <Link to={to}>{children}</Link>
  ) : (
    <a href={to}>{children}</a>
  );
};

const Table = ({ children }: ComponentProps<'table'>) => (
  <div className='min-w-0 overflow-x-auto rounded-md border'>
    <table className='[&_th]:bg-muted [&_th]:text-foreground w-full border-collapse text-left text-sm [&_td]:border-t [&_td]:px-4 [&_td]:py-3 [&_th]:px-4 [&_th]:py-3 [&_th]:font-semibold'>
      {children}
    </table>
  </div>
);

export const DocumentationMarkdown = ({
  children,
  id,
}: {
  children: string;
  id: string;
}) => (
  <div className='text-muted-foreground [&_a]:text-foreground [&_code]:text-foreground [&_h2]:text-foreground [&_h3]:text-foreground [&_h4]:text-foreground [&_pre]:bg-muted grid min-w-0 grid-cols-1 gap-4 [&_a]:underline [&_a]:decoration-current/30 [&_a]:underline-offset-4 [&_a:hover]:decoration-current [&_code]:font-mono [&_code]:text-[0.875em] [&_h2]:text-2xl [&_h2]:font-semibold [&_h2]:tracking-tight [&_h3]:mt-6 [&_h3]:text-xl [&_h3]:font-semibold [&_h3]:tracking-tight [&_h4]:mt-4 [&_h4]:text-lg [&_h4]:font-semibold [&_li+li]:mt-2 [&_ol]:list-decimal [&_ol]:pl-5 [&_pre]:overflow-x-auto [&_pre]:rounded-md [&_pre]:border [&_pre]:px-4 [&_pre]:py-3 [&_pre]:text-sm [&_pre]:leading-6 [&_pre_code]:text-sm [&_ul]:list-disc [&_ul]:pl-5'>
    <Markdown
      components={{
        a: DocumentationLink,
        h1: ({ children }) => (
          <h2 id={id} className='scroll-mt-6'>
            <a href={`#${id}`} className='no-underline! hover:underline!'>
              {children}
            </a>
          </h2>
        ),
        h2: 'h3',
        h3: 'h4',
        table: Table,
      }}
      remarkPlugins={[remarkGfm]}
    >
      {children}
    </Markdown>
  </div>
);
