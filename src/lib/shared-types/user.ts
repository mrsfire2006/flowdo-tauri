import type { UserSelect } from '$lib/db/schemas/auth.schema';

export type UserBio = Pick<UserSelect, 'id' | 'email' | 'name'>;
