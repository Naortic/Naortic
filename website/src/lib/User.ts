export interface UserOptions {
	name?: string;
	token?: string;
}

export class User {
	constructor(public data: UserOptions) {}

	public static from(data: string): this {
		return new User(JSON.parse(data));
	}

	public toString(): string {
		return JSON.stringify(this);
	}
}
