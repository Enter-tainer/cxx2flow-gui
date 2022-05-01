const generateGraphviz = async (code: string): Promise<string> => {
  return `digraph {
graph [splines=polyline];
D0 [shape=box, style=rounded, label="begin"];
{rank = sink; D1 [shape=box, style=rounded, label="end"];}
D4 [shape=box, label="int a = 0;"];
D6 [shape=box, label="int a = 1;"];
D4 -> D6;
D6 -> D1;
D0 -> D4;
}`
}

export { generateGraphviz }
