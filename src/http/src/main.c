#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <arpa/inet.h>
#include <stdbool.h>

#define PORT 8080
#define BUFFER_SIZE 4096

int main() {

	int sv_fd = socket(AF_INET, SOCK_STREAM, 0);
	struct sockaddr_in sv_addr;
	socklen_t sv_addrlen = sizeof(sv_addr);

	if (sv_fd == -1) {
		perror("socket failed");
		exit(EXIT_FAILURE);
	}

	sv_addr.sin_family = AF_INET;
	sv_addr.sin_addr.s_addr = INADDR_ANY;
	sv_addr.sin_port = htons(PORT);

	int opt = 1;
	if (setsockopt(sv_fd, SOL_SOCKET, SO_REUSEADDR, &opt, sizeof(opt)) < 0) {
		close(sv_fd);
		perror("setsockopt failed");
		exit(EXIT_FAILURE);
	}

	if (bind(sv_fd, (struct sockaddr*) &sv_addr, sizeof(sv_addr)) < 0) {
		close(sv_fd);
		perror("bind failed");
		exit(EXIT_FAILURE);
	}

	if (listen(sv_fd, 10) < 0) {
		close(sv_fd);
		perror("listen failed");
		exit(EXIT_FAILURE);
	}

	printf("listening on port %d\n", PORT);

	while (true) {

		char buffer[BUFFER_SIZE];

		int cl_fd = accept(sv_fd, (struct sockaddr*) &sv_addr, &sv_addrlen);

		if (cl_fd < 0) {
			perror("accept failed");

		} else {

			ssize_t recd = read(cl_fd, buffer, BUFFER_SIZE);

			if (recd < 0) {
				perror("client error");

			} else if (recd == 0) {
				
			} else if (recd > 0) {
				printf("====================\n");
				write(fileno(stdout), buffer, recd);
				printf("====================\n");
			}

			close(cl_fd);
		}
	}

	close(sv_fd);

	return 0;
}
