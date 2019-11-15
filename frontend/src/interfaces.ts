export interface FileItem {
    name: string;
    children: object[];
    has_backup_inside: boolean;
    included_in_backup: boolean;
}

export interface ConfigItem {
    backup_interval: { secs: number };
    config: { repo_password: string, repo_path: any };
    forget_rate: object;
    targets: { 
    	exclusions: string[],
    	folders: string[],
    	tags: string[]
    };
}

export interface BetterConfigItem {
    name: string;
    backup_interval: number;
    repo_password: string;
    repo_path: string;
    forget_rate: any;
    targets: Targets;
}

export interface Targets {
    exclusions: string[],
    folders: string[],
    tags: string[]
}