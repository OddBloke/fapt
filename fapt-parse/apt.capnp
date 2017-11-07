@0xaf696212bdf0eef6;

### Everything deals with streams of Items.

struct Item {
    union {
        end     @0 :Void;
        raw     @1 :RawPackage;
        index   @2 :RawIndex;
        package @3 :Package;
    }
}

### An unparsed, raw package

struct RawPackage {
    type    @0 :RawPackageType;
    entries @1 :List(Entry);
}

enum RawPackageType {
    source @0;
    binary @1;
}

struct Entry {
    key   @0 :Text;
    value @1 :Text;
}

### An unparsed, raw index

struct RawIndex {
    archive   @0 :Text;
    version   @1 :Text;
    origin    @2 :Text;
    codename  @3 :Text;
    label     @4 :Text;
    site      @5 :Text;
    component @6 :Text;
    arch      @7 :Text;
    type      @8 :Text;
}

### The parsed top-level types for package

struct Package {
    name     @0 :Text;
    version  @1 :Text;
    priority @2 :Priority;
    arch     @3 :List(Text);

    maintainer         @4 :List(Identity);
    originalMaintainer @5 :List(Identity);

    parseErrors        @6 :List(Text);
    unrecognisedFields @7 :List(Text);

    style :union {
        source @8  :Source;
        binary @9 :Binary;
    }
}

struct Source {
    format   @0 :SourceFormat;

    binaries @1 :List(SourceBinary);
    files    @2 :List(File);
    vcs      @3 :List(Vcs);

    buildDep           @4 :List(Dependency);
    buildDepArch       @5 :List(Dependency);
    buildDepIndep      @6 :List(Dependency);
    buildConflict      @7 :List(Dependency);
    buildConflictArch  @8 :List(Dependency);
    buildConflictIndep @9 :List(Dependency);

    uploaders @10 :List(Identity);

    unparsed  @11 :UnparsedSource;
}

struct Binary {
    file           @0 :File;

    essential      @1 :Bool;
    buildEssential @2 :Bool;

    installedSize  @3 :UInt64;

    description    @4 :Text;

    depends     @5 :List(Dependency);
    recommends  @6 :List(Dependency);
    suggests    @7 :List(Dependency);
    enhances    @8 :List(Dependency);
    preDepends  @9 :List(Dependency);

    breaks     @10 :List(Dependency);
    conflicts  @11 :List(Dependency);
    replaces   @12 :List(Dependency);

    provides   @13 :List(Dependency);

    unparsed   @14 :UnparsedBinary;
}

### The dependency chain types

struct Dependency {
    alternate @0 :List(SingleDependency);
}

struct SingleDependency {
    package            @0 :Text;
    arch               @1 :Text;
    versionConstraints @2 :List(Constraint);
    archFilter         @3 :List(Text);
    stageFilter        @4 :List(Text);
}

struct Constraint {
    version  @0 :Text;
    operator @1 :ConstraintOperator;
}

enum ConstraintOperator {
    ge @0;
    eq @1;
    le @2;
    gt @3;
    lt @4;
}

### Other types

struct File {
    name   @0 :Text;
    size   @1 :UInt64;
    md5    @2 :Text;
    sha1   @3 :Text;
    sha256 @4 :Text;
    sha512 @5 :Text;
}

struct Vcs {
    description @0 :Text;
    type        @1 :VcsType;
    tag         @2:VcsTag;
}

enum VcsType {
    browser @0;
    arch    @1;
    bzr     @2;
    cvs     @3;
    darcs   @4;
    git     @5;
    hg      @6;
    mtn     @7;
    svn     @8;
}

enum VcsTag {
    vcs      @0;
    orig     @1;
    debian   @2;
    upstream @3;
}

struct SourceBinary {
    name      @0 :Text;
    style     @1 :Text;
    section   @2 :Text;

    priority  @3 :Priority;
    extras    @4 :List(Text);
}

# https://www.debian.org/doc/debian-policy/#priorities
enum Priority {
    unknown   @0;
    required  @1;
    important @2;
    standard  @3;
    optional  @4;
    extra     @5;
    source    @6;
}

struct Description {
    locale @0 :Text;
    value  @1 :Text;
}

struct Identity {
    name  @0 :Text;
    email @1 :Text;
}

enum SourceFormat {
    unknown     @0;
    original    @1;
    quilt3dot0  @2;
    native3dot0 @3;
    git3dot0    @4;
}

## generated by gen.py

struct UnparsedSource {
    directory              @0 :Text;
    homepage               @1 :Text;
    standardsVersion       @2 :Text;
    section                @3 :Text;
    testsuite              @4 :Text;
    testsuiteTriggers      @5 :Text;
    testsuiteRestrictions  @6 :Text;
    autobuild              @7 :Text;
    dmUploadAllowed        @8 :Text;
    extraSourceOnly        @9 :Text;
    buildIndepArchitecture @10 :Text;
    dgit                   @11 :Text;
    goImportPath           @12 :Text;
    pythonVersion          @13 :Text;
    python3Version         @14 :Text;
    rubyVersions           @15 :Text;
}

struct UnparsedBinary {
    homepage               @0 :Text;
    section                @1 :Text;
    source                 @2 :Text;
    task                   @3 :Text;
    bugs                   @4 :Text;
    supported              @5 :Text;
    origin                 @6 :Text;
    status                 @7 :Text;
    buildIds               @8 :Text;
    multiArch              @9 :Text;
    packageType            @10 :Text;
    autoBuiltPackage       @11 :Text;
    builtUsing             @12 :Text;
    modaliases             @13 :Text;
    gstreamerDecoders      @14 :Text;
    gstreamerElements      @15 :Text;
    gstreamerEncoders      @16 :Text;
    gstreamerUriSinks      @17 :Text;
    gstreamerUriSources    @18 :Text;
    gstreamerVersion       @19 :Text;
    license                @20 :Text;
    vendor                 @21 :Text;
    goImportPath           @22 :Text;
    pythonVersion          @23 :Text;
    python3Version         @24 :Text;
    rubyVersions           @25 :Text;
    luaVersions            @26 :Text;
    pythonEggName          @27 :Text;
    ghcPackage             @28 :Text;
    nppApplications        @29 :Text;
    nppDescription         @30 :Text;
    nppFile                @31 :Text;
    nppMimetype            @32 :Text;
    nppName                @33 :Text;
    postgresqlCatversion   @34 :Text;
    postgresqlVersion      @35 :Text;
    tads2Version           @36 :Text;
    tads3Version           @37 :Text;
    xulAppid               @38 :Text;
    phasedUpdatePercentage @39 :Text;
    builtForProfiles       @40 :Text;
    class                  @41 :Text;
    conffiles              @42 :Text;
    configVersion          @43 :Text;
    files                  @44 :Text;
    important              @45 :Text;
    installerMenuItem      @46 :Text;
    kernelVersion          @47 :Text;
    msdosFilename          @48 :Text;
    optional               @49 :Text;
    packageRevision        @50 :Text;
    recommended            @51 :Text;
    revision               @52 :Text;
    subarchitecture        @53 :Text;
    tag                    @54 :Text;
    triggersAwaited        @55 :Text;
    triggersPending        @56 :Text;
}
