// use 🦀 allocator
void *alloc(int len);
void free(void *ptr);
void *realloc(void *ptr, unsigned int new_size);

int cadd(int x,int y) {
  return x+y;
}

char *chello(){
  char *hell=alloc(13);
  hell[0]='H';
  hell[1]='e';
  hell[2]='l';
  hell[3]='l';
  hell[4]='o';
  hell[5]=' ';
  hell[6]='f';
  hell[7]='r';
  hell[8]='o';
  hell[9]='m';
  hell[10]=' ';
  hell[11]='C';
  hell[12]=0;
  return hell;
}

void chello_free(char *chello) {
	free(chello);
}
