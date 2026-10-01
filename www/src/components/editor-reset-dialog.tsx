import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from '@/components/ui/dialog';
import { RotateCcw } from 'lucide-react';

interface EditorResetDialogProps {
  example: string;
  onReset: () => void;
}

export const EditorResetDialog = ({
  example,
  onReset,
}: EditorResetDialogProps) => (
  <Dialog>
    <DialogTrigger asChild>
      <Button
        variant='ghost'
        size='icon'
        className='dark:hover:bg-accent h-7 w-7 cursor-pointer'
        aria-label={`Reset to ${example} example`}
        title={`Reset to ${example} example`}
      >
        <RotateCcw className='h-4 w-4' aria-hidden='true' />
      </Button>
    </DialogTrigger>
    <DialogContent className='sm:max-w-[425px]'>
      <DialogHeader>
        <DialogTitle>Reset editor?</DialogTitle>
        <DialogDescription>
          This will replace your current editor contents with the {example}{' '}
          example program.
        </DialogDescription>
      </DialogHeader>
      <DialogFooter>
        <DialogClose asChild>
          <Button
            variant='outline'
            className='dark:border-input dark:bg-background dark:hover:bg-accent cursor-pointer'
          >
            Cancel
          </Button>
        </DialogClose>
        <DialogClose asChild>
          <Button
            variant='destructive'
            className='cursor-pointer'
            onClick={onReset}
          >
            Reset
          </Button>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  </Dialog>
);
