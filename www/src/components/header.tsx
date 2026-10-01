import { Button } from '@/components/ui/button';
import { Moon, Radius, Sun } from 'lucide-react';

interface HeaderProps {
  darkMode: boolean;
  onToggleTheme: () => void;
}

export const Header = ({ darkMode, onToggleTheme }: HeaderProps) => (
  <header className='flex shrink-0 items-center gap-6 px-4 py-3 sm:px-6'>
    <a
      href='/val'
      className='flex shrink-0 items-center gap-2 rounded-sm font-semibold focus-visible:outline-2 focus-visible:outline-offset-2'
    >
      <Radius className='h-4 w-4' aria-hidden='true' />
      val
    </a>
    <div className='ml-auto flex items-center gap-2 sm:gap-4'>
      <nav
        aria-label='Main navigation'
        className='flex items-center gap-4 text-sm'
      >
        <a
          href='/val'
          aria-current='page'
          className='rounded-sm font-medium focus-visible:outline-2 focus-visible:outline-offset-2'
        >
          Playground
        </a>
        <a
          href='https://github.com/terror/val'
          className='text-muted-foreground hover:text-foreground rounded-sm focus-visible:outline-2 focus-visible:outline-offset-2'
        >
          GitHub
        </a>
      </nav>
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
    </div>
  </header>
);
