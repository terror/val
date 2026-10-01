import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { ExternalLink, Info } from 'lucide-react';
import wasmUrl from 'val-wasm/val_bg.wasm?url';

export const PlaygroundInfoDialog = () => (
  <Dialog>
    <DialogTrigger asChild>
      <Button
        variant='ghost'
        size='icon'
        className='dark:hover:bg-accent h-7 w-7 cursor-pointer'
        aria-label='Information'
        title='Information'
      >
        <Info className='h-4 w-4' aria-hidden='true' />
      </Button>
    </DialogTrigger>
    <DialogContent
      aria-describedby={undefined}
      className='max-h-[calc(100dvh-2rem)] overflow-y-auto sm:max-w-[520px]'
    >
      <DialogTitle className='sr-only'>Parser information</DialogTitle>
      <dl className='grid gap-4 text-sm'>
        <div className='grid gap-1'>
          <dt className='text-muted-foreground text-xs font-medium'>Asset</dt>
          <dd className='font-mono text-xs break-all'>{wasmUrl}</dd>
        </div>
        <div className='grid gap-1'>
          <dt className='text-muted-foreground text-xs font-medium'>Path</dt>
          <dd className='font-mono text-xs break-all'>crates/val-wasm</dd>
        </div>
        <div className='grid gap-1'>
          <dt className='text-muted-foreground text-xs font-medium'>
            Repository
          </dt>
          <dd className='min-w-0'>
            <a
              href='https://github.com/terror/val'
              target='_blank'
              rel='noreferrer'
              className='inline-flex max-w-full cursor-pointer items-center gap-1 underline-offset-4 hover:underline'
            >
              <span className='truncate'>terror/val</span>
              <ExternalLink className='h-3 w-3 shrink-0' aria-hidden='true' />
            </a>
          </dd>
        </div>
      </dl>
    </DialogContent>
  </Dialog>
);
