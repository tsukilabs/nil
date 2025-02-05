import { message } from '@tauri-apps/plugin-dialog';

export function error(text: string) {
  message(text, { kind: 'error', title: 'Error' }).catch(console.error);
}

export function info(text: string) {
  message(text, { kind: 'info', title: 'Info' }).catch(console.error);
}

export function warn(text: string) {
  message(text, { kind: 'warning', title: 'Warning' }).catch(console.error);
}
