import java.util.HashSet;
import java.util.List;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

public class log_table {
  public static void main(String[] args) {
    IntStream.rangeClosed(2, 3)
      .forEach(x -> { System.out.print("| zeta = "+x);
        List<Integer> seq = IntStream.rangeClosed(0, 29)
          .mapToObj(i -> (int) (Math.pow(x, i) % 31))
          .collect(Collectors.toList());
        seq.forEach(i -> System.out.print(" | "+i));
        System.out.print(" | \n");
        if (seq.stream().allMatch(new HashSet<>()::add)) System.out.println("Found prime element zeta = "+x);
      });
  }
}
