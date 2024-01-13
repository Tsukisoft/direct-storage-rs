// Missing constants from https://learn.microsoft.com/en-us/windows/win32/dstorage/dstorage-constants

using Windows.Win32.Foundation;
using Windows.Win32.Foundation.Metadata;

namespace Microsoft.Direct3D.DirectStorage
{
    public static partial class Apis
    {
        public const uint FACILITY_GAME = 2340u;

        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_ACCESS_VIOLATION = -1994129399;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_ALREADY_RUNNING = -1994129407;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_BCPACK_BAD_DATA = -1994129355;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_BCPACK_BAD_HEADER = -1994129356;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_COMPRESSED_DATA_TOO_LARGE = -1994129351;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_DECOMPRESSION_ERROR = -1994129360;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_DECRYPTION_ERROR = -1994129354;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_DEPRECATED_PREVIEW_GDK = -1994129384;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_END_OF_FILE = -1994129401;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_FILE_NOT_OPEN = -1994129397;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_FILE_TOO_FRAGMENTED = -1994129352;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_FILEBUFFERING_REQUIRES_DISABLED_BYPASSIO = -1994129343;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INDEX_BOUND = -1994129387;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_BCPACK_MODE = -1994129395;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_CLUSTER_SIZE = -1994129391;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_DESTINATION_SIZE = -1994129393;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_DESTINATION_TYPE = -1994129344;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_FENCE = -1994129374;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_FILE_HANDLE = -1994129385;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_FILE_OFFSET = -1994129382;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_INTERMEDIATE_SIZE = -1994129380;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_MEMORY_QUEUE_PRIORITY = -1994129372;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_QUEUE_CAPACITY = -1994129405;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_QUEUE_PRIORITY = -1994129389;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_SOURCE_TYPE = -1994129381;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_STAGING_BUFFER_SIZE = -1994129376;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_STATUS_ARRAY = -1994129373;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_INVALID_SWIZZLE_MODE = -1994129394;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_IO_TIMEOUT = -1994129386;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_NOT_RUNNING = -1994129406;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_PASSTHROUGH_ERROR = -1994129353;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_QUEUE_CLOSED = -1994129392;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_REQUEST_TOO_LARGE = -1994129400;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_RESERVED_FIELDS = -1994129396;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_STAGING_BUFFER_LOCKED = -1994129377;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_STAGING_BUFFER_TOO_SMALL = -1994129375;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_SYSTEM_NOT_SUPPORTED = -1994129379;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_TOO_MANY_FILES = -1994129388;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_TOO_MANY_QUEUES = -1994129390;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_UNSUPPORTED_FILE = -1994129398;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_UNSUPPORTED_VOLUME = -1994129403;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_XVD_DEVICE_NOT_SUPPORTED = -1994129404;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_XVD_NOT_REGISTERED = -1994129383;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_ZLIB_BAD_DATA = -1994129358;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_ZLIB_BAD_HEADER = -1994129359;
        [NativeTypeName("HRESULT")]
        public const int E_DSTORAGE_ZLIB_PARITY_FAIL = -1994129357;
    }
}
