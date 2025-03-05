export type NewUser = {
	email: string;
	password: string;
	first_name: string;
	last_name: string;
	student_id: string;
	university: string;
	verified_email: boolean;
};

export const newUserState: NewUser = $state({
	email: '',
	password: '',
	first_name: '',
	last_name: '',
	student_id: '',
	university: '',
	verified_email: false
});
