export type ToastType = 'success' | 'info' | 'warning' | 'error';

export interface Toast {
  id: string;
  type: ToastType;
  message: string;
  showIcon?: boolean;
  duration?: number;
  onConfirm?: () => void;
  confirmText?: string;
}

export interface ToastOptions {
  type: ToastType;
  message: string;
  showIcon?: boolean;
  duration?: number;
  onConfirm?: () => void;
  confirmText?: string;
}
