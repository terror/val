import { Button } from '@/components/ui/button';
import { useMediaQuery } from '@/hooks/use-media-query';
import { cn } from '@/lib/utils';
import * as Dialog from '@radix-ui/react-dialog';
import { Menu, Moon, Radius, Sun } from 'lucide-react';
import { NavLink } from 'react-router';

const links = [
  { href: '/documentation', label: 'Documentation' },
  { href: '/playground', label: 'Playground' },
  { href: 'https://github.com/terror/val', label: 'GitHub' },
];

interface HeaderProps {
  darkMode: boolean;
  onToggleTheme: () => void;
}

const NavigationLinks = ({ mobile = false }: { mobile?: boolean }) =>
  links.map(({ href, label }) => {
    const className = cn(
      'text-muted-foreground hover:text-foreground aria-[current=page]:text-foreground rounded-sm focus-visible:outline-2 focus-visible:outline-offset-2 aria-[current=page]:font-medium',
      mobile && 'hover:bg-accent block px-3 py-3'
    );

    const link = href.startsWith('/') ? (
      <NavLink key={href} to={href} className={className}>
        {label}
      </NavLink>
    ) : (
      <a key={href} href={href} className={className}>
        {label}
      </a>
    );

    return mobile ? (
      <Dialog.Close key={href} asChild>
        {link}
      </Dialog.Close>
    ) : (
      link
    );
  });

export const Header = ({ darkMode, onToggleTheme }: HeaderProps) => {
  const wide = useMediaQuery('(min-width: 640px)');

  return (
    <header className='flex shrink-0 items-center gap-6 px-4 py-3 sm:px-6'>
      <NavLink
        to='/'
        end
        className='flex shrink-0 items-center gap-2 rounded-sm font-semibold focus-visible:outline-2 focus-visible:outline-offset-2'
      >
        <Radius className='h-4 w-4' aria-hidden='true' />
        val
      </NavLink>
      <div className='ml-auto flex items-center gap-2 sm:gap-4'>
        {wide && (
          <nav
            aria-label='Main navigation'
            className='flex items-center gap-4 text-sm'
          >
            <NavigationLinks />
          </nav>
        )}
        <Button
          variant='ghost'
          size='icon'
          className='h-10 w-10 cursor-pointer sm:h-8 sm:w-8'
          onClick={onToggleTheme}
          aria-label={darkMode ? 'Switch to light mode' : 'Switch to dark mode'}
          aria-pressed={darkMode}
          title={darkMode ? 'Switch to light mode' : 'Switch to dark mode'}
        >
          {darkMode ? <Sun aria-hidden='true' /> : <Moon aria-hidden='true' />}
        </Button>
        {!wide && (
          <div className='relative'>
            <Dialog.Root modal={false}>
              <Dialog.Trigger asChild>
                <Button
                  variant='ghost'
                  size='icon'
                  className='h-10 w-10 cursor-pointer'
                  aria-label='Toggle navigation'
                >
                  <Menu aria-hidden='true' />
                </Button>
              </Dialog.Trigger>
              <Dialog.Content
                aria-describedby={undefined}
                className='bg-popover text-popover-foreground absolute top-full right-0 z-50 mt-2 w-52 rounded-md border p-1 shadow-lg'
              >
                <Dialog.Title className='sr-only'>Navigation</Dialog.Title>
                <nav aria-label='Main navigation' className='text-sm'>
                  <NavigationLinks mobile />
                </nav>
              </Dialog.Content>
            </Dialog.Root>
          </div>
        )}
      </div>
    </header>
  );
};
