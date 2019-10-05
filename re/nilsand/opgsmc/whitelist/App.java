package re.nilsand.opgsmc.whitelist;

public class App extends org.bukkit.plugin.java.JavaPlugin {

	static {
		System.loadLibrary("http_server");
	}

    @Override
    public native void onEnable();
}
