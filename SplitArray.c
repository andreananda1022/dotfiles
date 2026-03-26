#include <stdio.h>

bool isPossibleToSplit(int num[], int numSize);

int main() {
  int arr[] = {1, 4, 2, 2, 1, 3};
  int size = sizeof(arr) / sizeof(arr[0]);

  printf("isPossible: %s\n", isPossibleToSplit(array1, size1) ? "True" : "false");

  return 0;
}

bool isPossibleToSplit(int num[], int numSize) {
  int frekuensi[10] = {0};

  for (int i = 0; i < numSize; i++) {
    int angkaSaatIni = num[i];

    frekuensi[angkaSaatIni]++;

    if (frekuensi[angkaSaatIni] > 2) {
      return false;
    }
  }

  return true;
}
