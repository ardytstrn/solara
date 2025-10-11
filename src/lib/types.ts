export type ActivePage = 'accounts' | 'instances' | 'mods' | 'performance' | 'settings';

export type AccountType = 'offline' | 'microsoft';

export interface UserProfile {
    uuid: string;
    username: string;
    avatarUrl: string;
    accountType: AccountType;
}