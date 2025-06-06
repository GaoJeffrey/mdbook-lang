import java.lang.reflect.Method;
public class MainClassFinder{
	public static void main(String args[]){
        try {
            java.net.URL classUrl = new java.io.File(args[0]).toURI().toURL();
            if(args.length < 1 || classUrl == null){
                System.out.println("no_main");
                return;
            }
            String className = new java.io.File(args[0]).getName().replaceAll("\\.class$", "");
            ClassLoader loader = new java.net.URLClassLoader(new java.net.URL[]{classUrl});
            Class<?> myClass = Class.forName(className, false, loader);
            Method m = myClass.getDeclaredMethod("main", String[].class);
            if (m == null) {
                System.out.println("no_main");
            }else{
                if(m.toString().startsWith("public static void")){
                    System.out.println("have_main");
                }else System.out.println("no_main");
            }
        } catch (Exception e) {
            System.out.println("no_main");
        }
	}
}