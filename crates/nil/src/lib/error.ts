import * as dialog from '@/lib/dialog';

export function handleError(err: unknown) {
  console.error(err);
  if (err instanceof Error) {
    dialog.error(err.message);
  } else {
    dialog.error(String(err));
  }
}
