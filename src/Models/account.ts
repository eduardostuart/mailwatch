export type Account = {
  id: string;
  name: string;
  server: string;
  port: number;
  color: string;
  active: boolean;
  username: string;
  mailbox: string;
  password?: string;
};
