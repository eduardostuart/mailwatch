declare module "uuid";

type Account = {
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

type ConnectionCreds = {
  server: string;
  port: number;
  username: string;
  password: string;
  mailbox: string;
};
