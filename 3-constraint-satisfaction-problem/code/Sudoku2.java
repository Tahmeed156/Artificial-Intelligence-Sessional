import org.chocosolver.solver.Model;
import org.chocosolver.solver.Solver;
import org.chocosolver.solver.variables.IntVar;

public class Sudoku2 {

    public void modelAndSolve() {
        
        Model model = new Model("Sudoku");
        
        IntVar[][] rows = model.intVarMatrix("rows", 9, 9, 1, 9);
        IntVar[][] cols = model.intVarMatrix("cols", 9, 9, 1, 9);
        IntVar[][] boxes = model.intVarMatrix("boxes", 9, 9, 1, 9);
        
        for(int i=0;i<9;i++){
            model.allDifferent(rows[i]).post();
            model.allDifferent(cols[i]).post();
            model.allDifferent(boxes[i]).post();
            
        }
        
        for(int i=0;i<9;i++){
            for(int j=0;j<9;j++){
                model.arithm(rows[i][j], "=", cols[j][i]).post();
                int boxNo = (i/3) * 3 + (j/3);
                int boxPosition = (i%3) * 3 + (j%3);
                model.arithm(rows[i][j], "=", boxes[boxNo][boxPosition]).post();
            }
        }
        
        model.arithm(rows[0][2], "=", 2).post();
        
        model.arithm(rows[1][1], "=", 8).post();
        model.arithm(rows[1][4], "=", 3).post();
        model.arithm(rows[1][7], "=", 7).post();
        
        model.arithm(rows[2][0], "=", 3).post();
        model.arithm(rows[2][3], "=", 5).post();
        model.arithm(rows[2][5], "=", 4).post();
        
        model.arithm(rows[3][7], "=", 2).post();
        model.arithm(rows[3][8], "=", 8).post();
        
        model.arithm(rows[4][0], "=", 8).post();
        model.arithm(rows[4][1], "=", 3).post();
        model.arithm(rows[4][4], "=", 1).post();
        
        
        model.arithm(rows[5][1], "=", 4).post();
        model.arithm(rows[5][3], "=", 7).post();
        model.arithm(rows[5][6], "=", 3).post();
        model.arithm(rows[5][7], "=", 5).post();
        model.arithm(rows[5][8], "=", 1).post();
        
        model.arithm(rows[6][1], "=", 7).post();
        model.arithm(rows[6][4], "=", 5).post();
        model.arithm(rows[6][5], "=", 6).post();
        model.arithm(rows[6][8], "=", 4).post();
        
        model.arithm(rows[7][2], "=", 3).post();
        
        model.arithm(rows[8][2], "=", 5).post();
        model.arithm(rows[8][3], "=", 4).post();
        model.arithm(rows[8][5], "=", 1).post();
        model.arithm(rows[8][6], "=", 6).post();
        
        
        
        Solver solver = model.getSolver();
        // solver.showStatistics();
        // solver.showSolutions();
        solver.findSolution();
        
        
        for(int i=0;i<9;i++){
            for(int j=0;j<9;j++){
                int k = rows[i][j].getValue();
                System.out.print(k + " ");
            }
            System.out.println();
        }
        
    }

    public static void main(String[] args) {
        new Sudoku2().modelAndSolve();
    }
}
