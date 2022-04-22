![Image description](https://www.wasm.builders/images/Zc77YM--Xwgm0mQVcit_NQKiYBfKmYc5810FlZjBzMc/s:1000:420/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy84eXpx/M3lvZWFjajhtZ2xw/eWxkMi5wbmc)


In this blog, we'll be compiling and executing Breadth First Search(BFS) algorithm in C using wasmtime.
## Some tools required for the program to run:
### 1. Install C/Cpp compiler


```
gcc -v
make -v
```



### 2. Install Wasmer


```
$ curl https://get.wasmer.io -sSfL | sh
```


### 3. Install Wasienv


```
curl https://raw.githubusercontent.com/wasienv/wasienv/master/install.sh | sh
wasmtime ReBinary.wasm 
```


### 4. Install Wasmtime


```
$ curl https://wasmtime.dev/install.sh -sSf | bash
$ wasienv install-sdk unstable
```


## Compiling BFS Algorithm in C
Create a file named BFS.c and add the following BFS code to it:


```
#include <stdio.h>
#include <stdlib.h>
#define SIZE 40

struct queue {
  int items[SIZE];
  int front;
  int rear;
};

struct queue* createQueue();
void enqueue(struct queue* q, int);
int dequeue(struct queue* q);
void display(struct queue* q);
int isEmpty(struct queue* q);
void printQueue(struct queue* q);

struct node {
  int vertex;
  struct node* next;
};

struct node* createNode(int);

struct Graph {
  int numVertices;
  struct node** adjLists;
  int* visited;
};


void bfs(struct Graph* graph, int startVertex) {
  struct queue* q = createQueue();

  graph->visited[startVertex] = 1;
  enqueue(q, startVertex);

  while (!isEmpty(q)) {
    printQueue(q);
    int currentVertex = dequeue(q);
    printf("Visited %d\n", currentVertex);

    struct node* temp = graph->adjLists[currentVertex];

    while (temp) {
      int adjVertex = temp->vertex;

      if (graph->visited[adjVertex] == 0) {
        graph->visited[adjVertex] = 1;
        enqueue(q, adjVertex);
      }
      temp = temp->next;
    }
  }
}

struct node* createNode(int v) {
  struct node* newNode = malloc(sizeof(struct node));
  newNode->vertex = v;
  newNode->next = NULL;
  return newNode;
}

struct Graph* createGraph(int vertices) {
  struct Graph* graph = malloc(sizeof(struct Graph));
  graph->numVertices = vertices;

  graph->adjLists = malloc(vertices * sizeof(struct node*));
  graph->visited = malloc(vertices * sizeof(int));

  int i;
  for (i = 0; i < vertices; i++) {
    graph->adjLists[i] = NULL;
    graph->visited[i] = 0;
  }

  return graph;
}


void addEdge(struct Graph* graph, int src, int dest) {
 
  struct node* newNode = createNode(dest);
  newNode->next = graph->adjLists[src];
  graph->adjLists[src] = newNode;

  
  newNode = createNode(src);
  newNode->next = graph->adjLists[dest];
  graph->adjLists[dest] = newNode;
}


struct queue* createQueue() {
  struct queue* q = malloc(sizeof(struct queue));
  q->front = -1;
  q->rear = -1;
  return q;
}


int isEmpty(struct queue* q) {
  if (q->rear == -1)
    return 1;
  else
    return 0;
}


void enqueue(struct queue* q, int value) {
  if (q->rear == SIZE - 1)
    printf("\nQueue is Full!!");
  else {
    if (q->front == -1)
      q->front = 0;
    q->rear++;
    q->items[q->rear] = value;
  }
}


int dequeue(struct queue* q) {
  int item;
  if (isEmpty(q)) {
    printf("Queue is empty");
    item = -1;
  } else {
    item = q->items[q->front];
    q->front++;
    if (q->front > q->rear) {
      printf("Resetting queue ");
      q->front = q->rear = -1;
    }
  }
  return item;
}


void printQueue(struct queue* q) {
  int i = q->front;

  if (isEmpty(q)) {
    printf("Queue is empty");
  } else {
    printf("\nQueue contains \n");
    for (i = q->front; i < q->rear + 1; i++) {
      printf("%d ", q->items[i]);
    }
  }
}

int main() {
  struct Graph* graph = createGraph(6);
  addEdge(graph, 0, 1);
  addEdge(graph, 0, 2);
  addEdge(graph, 1, 2);
  addEdge(graph, 1, 4);
  addEdge(graph, 1, 3);
  addEdge(graph, 2, 4);
  addEdge(graph, 3, 4);

  bfs(graph, 0);

  return 0;
}
```



### Compile C code using wasicc:


```
$ wasicc BFS.c -o ReBinary.wasm
```

![Image description](https://www.wasm.builders/images/Itz6IqkoJAOK1Q1GKymgAmX3wcsB34_F9ux2edQBAgw/w:880/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy82emNv/bzQzNHpycTc2YzI4/Z2Nmei5wbmc)


Ignore the warnings.

## Run the program:


```
$ wasmtime ReBinary.wasm 
```

![Image description](https://www.wasm.builders/images/Itz6IqkoJAOK1Q1GKymgAmX3wcsB34_F9ux2edQBAgw/w:880/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy82emNv/bzQzNHpycTc2YzI4/Z2Nmei5wbmc)

Hope you enjoyed! 
