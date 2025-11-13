

#include <stdio.h>

struct Client {
	char* name;
};

void __Client_greet(struct Client* self) {
	printf("hello %s\n", self->name);
}

static struct {
	void (*greet)(struct Client*);
} Client = {
	__Client_greet,
};



int main(void) {

	struct Client client = {
		"Lucas"
	};

	Client.greet(&client);


	return 0;
}

