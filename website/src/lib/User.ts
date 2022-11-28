export interface UserOptions {
	name?: string;
	password?: string;
	token?: string;
	email?: string;
	friends?: string[];
}

export class User {
	constructor(public data: UserOptions) {}

	public static from(data: string): User {
		return new User(JSON.parse(data).data);
	}

	public toString(): string {
		return JSON.stringify(this);
	}
}
