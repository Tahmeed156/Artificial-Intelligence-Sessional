import org.chocosolver.solver.Model;
import org.chocosolver.solver.Solver;
import org.chocosolver.solver.variables.IntVar;

public class KenKen {

    public void modelAndSolve() {
        
        Model model = new Model("KenKen");
        
        IntVar[][] rows = model.intVarMatrix("rows", 6, 6, 1, 6);
        IntVar[][] cols = model.intVarMatrix("cols", 6, 6, 1, 6);
        
        for(int i=0; i<6; i++){
            model.allDifferent(rows[i]).post();
            model.allDifferent(cols[i]).post();
        }
        
        for(int i=0; i<6; i++){
            for(int j=0; j<6; j++){
                model.arithm(rows[i][j], "=", cols[j][i]).post();
            }
        }

        // Define cages
        IntVar[] cage1 = new IntVar[] {rows[0][0], rows[1][0], rows[0][1]};
        IntVar[] cage2 = new IntVar[] {rows[2][0], rows[3][0], rows[4][0]};
        IntVar[] cage3 = new IntVar[] {rows[5][0], rows[5][1]};
        IntVar[] cage4 = new IntVar[] {rows[1][1], rows[2][1]};
        IntVar[] cage5 = new IntVar[] {rows[3][1], rows[4][1]};
        IntVar[] cage6 = new IntVar[] {rows[0][2], rows[0][3], rows[1][2]};
        IntVar[] cage7 = new IntVar[] {rows[2][2], rows[3][2]};
        IntVar[] cage8 = new IntVar[] {rows[4][2], rows[4][3]};
        IntVar[] cage9 = new IntVar[] {rows[5][2], rows[5][3]};
        IntVar[] cage10 = new IntVar[] {rows[2][3], rows[3][3], rows[3][4]};
        IntVar[] cage11 = new IntVar[] {rows[0][4], rows[1][3], rows[1][4], rows[2][4]};
        IntVar[] cage12 = new IntVar[] {rows[4][4], rows[5][4]};
        IntVar[] cage13 = new IntVar[] {rows[0][5], rows[1][5]};
        IntVar[] cage14 = new IntVar[] {rows[2][5], rows[3][5]};
        IntVar[] cage15 = new IntVar[] {rows[4][5], rows[5][5]};

        // Cage constraints

        // Multiplications
        IntVar aux1 = model.intVar("aux1", 1, 36);
        model.times(cage1[0], cage1[1], aux1).post();
        model.times(cage1[2], aux1, 10).post();
        IntVar aux2 = model.intVar("aux2", 1, 36);
        model.times(cage2[0], cage2[1], aux2).post();
        model.times(cage2[2], aux2, 72).post();
        IntVar aux3 = model.intVar("aux3", 1, 36);
        model.times(cage6[0], cage6[1], aux3).post();
        model.times(cage6[2], aux3, 48).post();
        model.times(cage9[0], cage9[1], 30).post();

        // Additions
        model.sum(cage10, "=", 10).post();
        model.sum(cage11, "=", 14).post();
        model.sum(cage15, "=", 8).post();

        // Subtractions
        IntVar diff1 = cage4[0].dist(cage4[1]).intVar();
        model.arithm(diff1, "=", 1).post();
        IntVar diff2 = cage8[0].dist(cage8[1]).intVar();
        model.arithm(diff2, "=", 4).post();
        IntVar diff3 = cage13[0].dist(cage13[1]).intVar();
        model.arithm(diff3, "=", 2).post();
        IntVar diff4 = cage14[0].dist(cage14[1]).intVar();
        model.arithm(diff4, "=", 3).post();

        // Division
        IntVar div1 = model.intVar("div1", 2);
        div1.eq(cage5[0].div(cage5[1])).or(div1.eq(cage5[1].div(cage5[0]))).post(); 
        IntVar div2 = model.intVar("div2", 3);
        div2.eq(cage3[0].div(cage3[1])).or(div2.eq(cage3[1].div(cage3[0]))).post(); 
        IntVar div3 = model.intVar("div3", 3);
        div3.eq(cage7[0].div(cage7[1])).or(div3.eq(cage7[1].div(cage7[0]))).post(); 
        IntVar div4 = model.intVar("div4", 2);
        div4.eq(cage12[0].div(cage12[1])).or(div4.eq(cage12[1].div(cage12[0]))).post(); 

        // Solve

        Solver solver = model.getSolver();
        solver.findSolution();

        for(int i=0; i<6; i++){
            for(int j=0; j<6; j++){
                System.out.print(rows[i][j].getValue() + " ");
            }
            System.out.println();
        }
    }

    public static void main(String[] args) {
        new KenKen().modelAndSolve();
    }

}
