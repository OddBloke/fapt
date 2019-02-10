use std::cmp;
use std::collections::HashMap;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

use deb_version::compare_versions;
use failure::bail;
use failure::format_err;
use failure::Error;
use insideout::InsideOut;

use super::deps;
use super::rfc822;
use super::src;

/// The parsed top-level types for package
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PackageType {
    Source(Source),
    Binary(Binary),
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub priority: Priority,
    pub arches: Arches,
    pub section: String,

    pub maintainer: Vec<Identity>,
    pub original_maintainer: Vec<Identity>,

    pub homepage: Option<String>,

    pub unparsed: HashMap<String, Vec<String>>,

    pub style: PackageType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Source {
    pub format: SourceFormat,

    pub binaries: Vec<SourceBinary>,
    pub files: Vec<src::SourceArchive>,
    pub vcs: Vec<Vcs>,

    pub directory: String,
    pub standards_version: String,

    pub build_dep: Vec<Dependency>,
    pub build_dep_arch: Vec<Dependency>,
    pub build_dep_indep: Vec<Dependency>,
    pub build_conflict: Vec<Dependency>,
    pub build_conflict_arch: Vec<Dependency>,
    pub build_conflict_indep: Vec<Dependency>,

    pub uploaders: Vec<Identity>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Binary {
    // "File" is missing in e.g. dpkg/status, but never in Packages as far as I've seen
    pub file: Option<File>,

    pub essential: bool,
    pub build_essential: bool,

    pub installed_size: u64,

    pub description: String,
    pub source: Option<String>,
    pub status: Option<String>,

    pub depends: Vec<Dependency>,
    pub recommends: Vec<Dependency>,
    pub suggests: Vec<Dependency>,
    pub enhances: Vec<Dependency>,
    pub pre_depends: Vec<Dependency>,

    pub breaks: Vec<Dependency>,
    pub conflicts: Vec<Dependency>,
    pub replaces: Vec<Dependency>,

    pub provides: Vec<Dependency>,
}

// The dependency chain types

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dependency {
    pub alternate: Vec<SingleDependency>,
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SingleDependency {
    pub package: String,
    pub arch: Option<Arch>,
    /// Note: It's possible Debian only supports a single version constraint.
    pub version_constraints: Vec<Constraint>,
    pub arch_filter: Arches,
    pub stage_filter: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Constraint {
    pub version: String,
    pub operator: ConstraintOperator,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ConstraintOperator {
    Ge,
    Eq,
    Le,
    Gt,
    Lt,
}

// Other types

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub enum Arch {
    Any,
    All,
    Amd64,
    Armel,
    Armhf,
    Arm64,
    I386,
    Ia64,
    KFreeBsdAmd64,
    KFreeBsdI386,
    Mips,
    Mipsel,
    Mips64,
    Mips64El,
    Ppc64El,
    S390X,
    LinuxAny,
    X32,
    ParserBug,
}

pub type Arches = HashSet<Arch>;

impl FromStr for Arch {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Error> {
        Ok(match s {
            "all" => Arch::All,
            "any" => Arch::Any,
            "amd64" => Arch::Amd64,
            "armel" => Arch::Armel,
            "armhf" => Arch::Armhf,
            "arm64" => Arch::Arm64,
            "ia64" => Arch::Ia64,
            "i386" => Arch::I386,
            "kfreebsd-amd64" => Arch::KFreeBsdAmd64,
            "kfreebsd-i386" => Arch::KFreeBsdI386,
            "mips" => Arch::Mips,
            "mipsel" => Arch::Mipsel,
            "mips64" => Arch::Mips64,
            "mips64el" => Arch::Mips64El,
            "ppc64el" => Arch::Ppc64El,
            "s390x" => Arch::S390X,
            "linux-any" => Arch::LinuxAny,
            "x32" => Arch::X32,
            other => bail!("unrecognised arch: {:?}", other),
        })
    }
}

impl fmt::Display for Arch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Arch::All => "all",
                Arch::Any => "any",
                Arch::Amd64 => "amd64",
                Arch::Armel => "armel",
                Arch::Armhf => "armhf",
                Arch::Arm64 => "arm64",
                Arch::I386 => "i386",
                Arch::Ia64 => "ia64",
                Arch::KFreeBsdAmd64 => "kfreebsd-amd64",
                Arch::KFreeBsdI386 => "kfreebsd-i386",
                Arch::Mips => "mips",
                Arch::Mipsel => "mipsel",
                Arch::Mips64 => "mips64",
                Arch::Mips64El => "mips64el",
                Arch::Ppc64El => "ppc64el",
                Arch::S390X => "s390x",
                Arch::LinuxAny => "linux-any",
                Arch::X32 => "x32",
                Arch::ParserBug => return Err(fmt::Error {}),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub sha512: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Vcs {
    pub description: String,
    pub vcs: VcsType,
    pub tag: VcsTag,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VcsType {
    Browser,
    Arch,
    Bzr,
    Cvs,
    Darcs,
    Git,
    Hg,
    Mtn,
    Svn,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VcsTag {
    Vcs,
    Orig,
    Debian,
    Upstream,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceBinary {
    pub name: String,
    pub style: String,
    pub section: String,

    pub priority: Priority,
    pub extras: Vec<String>,
}

/// https://www.debian.org/doc/debian-policy/#priorities
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Priority {
    Unknown,
    Required,
    Important,
    Standard,
    Optional,
    Extra,
    Source,
}

impl Default for Priority {
    fn default() -> Self {
        Priority::Unknown
    }
}

pub struct Description {
    pub locale: String,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub name: String,
    pub email: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum SourceFormat {
    Original,
    Quilt3dot0,
    Native3dot0,
    Git3dot0,
}

impl Package {
    pub fn parse(map: &mut rfc822::Map) -> Result<Package, Error> {
        let style = if map.contains_key("Binary") {
            // Binary indicates that it's a source package *producing* that binary
            PackageType::Source(parse_src(map)?)
        } else {
            PackageType::Binary(parse_bin(map)?)
        };

        parse_pkg(map, style)
    }

    pub fn bin(&self) -> Option<&Binary> {
        match &self.style {
            PackageType::Binary(bin) => Some(&bin),
            _ => None,
        }
    }
}

fn parse_pkg(map: &mut rfc822::Map, style: PackageType) -> Result<Package, Error> {
    let arches = map
        .take_one_line("Architecture")?
        // TODO: alternate splitting rules?
        .split_whitespace()
        .map(|s| s.parse())
        .collect::<Result<HashSet<Arch>, Error>>()?;

    let original_maintainer = map
        .remove_one_line("Original-Maintainer")?
        .map(|line| super::ident::read(line))
        .inside_out()?
        .unwrap_or_else(Vec::new);

    Ok(Package {
        name: map.take_one_line("Package")?.to_string(),
        version: map.take_one_line("Version")?.to_string(),
        priority: super::parse_priority(map.take_one_line("Priority")?)?,
        arches,
        section: map.take_one_line("Section")?.to_string(),
        maintainer: super::ident::read(map.take_one_line("Maintainer")?)?,
        original_maintainer,
        homepage: map.remove_one_line("Homepage")?.map(|s| s.to_string()),
        style,
        unparsed: map
            .into_iter()
            .map(|(k, v)| {
                (
                    k.to_string(),
                    v.into_iter().map(|v| v.to_string()).collect(),
                )
            })
            .collect(),
    })
}

fn parse_src(map: &mut rfc822::Map) -> Result<Source, Error> {
    Ok(Source {
        format: src::parse_format(map.take_one_line("Format")?)?,
        binaries: src::take_package_list(map)?,
        files: src::take_files(map)?,
        directory: map.take_one_line("Directory")?.to_string(),
        vcs: super::vcs::extract(map)?,
        standards_version: map.take_one_line("Standards-Version")?.to_string(),
        build_dep: parse_dep(&map.remove("Build-Depends").unwrap_or_else(Vec::new))?,
        build_dep_arch: parse_dep(&map.remove("Build-Depends-Arch").unwrap_or_else(Vec::new))?,
        build_dep_indep: parse_dep(&map.remove("Build-Depends-Indep").unwrap_or_else(Vec::new))?,
        build_conflict: parse_dep(&map.remove("Build-Conflicts").unwrap_or_else(Vec::new))?,
        build_conflict_arch: parse_dep(
            &map.remove("Build-Conflicts-Arch").unwrap_or_else(Vec::new),
        )?,
        build_conflict_indep: parse_dep(
            &map.remove("Build-Conflicts-Indep").unwrap_or_else(Vec::new),
        )?,
        uploaders: map
            .remove_one_line("Uploaders")?
            .map(|line| super::ident::read(line))
            .inside_out()?
            .unwrap_or_else(Vec::new),
    })
}

fn parse_bin(it: &mut rfc822::Map) -> Result<Binary, Error> {
    // TODO: clearly `parse_file` is supposed to be called here somewhere
    let file = None;

    // TODO: this is missing in a couple of cases in dpkg/status; pretty crap
    let installed_size = match it.remove("Installed-Size") {
        Some(v) => rfc822::one_line(&v)?.parse()?,
        None => 0,
    };

    let essential = it
        .remove_one_line("Essential")?
        .map(|line| super::yes_no(line))
        .inside_out()?
        .unwrap_or(false);

    let build_essential = it
        .remove_one_line("Build-Essential")?
        .map(|line| super::yes_no(line))
        .inside_out()?
        .unwrap_or(false);

    Ok(Binary {
        file,
        essential,
        build_essential,
        installed_size,
        description: rfc822::joined(&it.take_err("Description")?),
        source: it.remove_one_line("Source")?.map(|s| s.to_string()),
        status: it.remove_one_line("Status")?.map(|s| s.to_string()),
        depends: parse_dep(&it.remove("Depends").unwrap_or_else(Vec::new))?,
        recommends: parse_dep(&it.remove("Recommends").unwrap_or_else(Vec::new))?,
        suggests: parse_dep(&it.remove("Suggests").unwrap_or_else(Vec::new))?,
        enhances: parse_dep(&it.remove("Enhances").unwrap_or_else(Vec::new))?,
        pre_depends: parse_dep(&it.remove("Pre-Depends").unwrap_or_else(Vec::new))?,
        breaks: parse_dep(&it.remove("Breaks").unwrap_or_else(Vec::new))?,
        conflicts: parse_dep(&it.remove("Conflicts").unwrap_or_else(Vec::new))?,
        replaces: parse_dep(&it.remove("Replaces").unwrap_or_else(Vec::new))?,
        provides: parse_dep(&it.remove("Provides").unwrap_or_else(Vec::new))?,
    })
}

pub trait RfcMapExt {
    fn get(&self, key: &str) -> Option<&Vec<&str>>;
    fn remove(&mut self, key: &str) -> Option<Vec<&str>>;

    fn take_err(&mut self, key: &str) -> Result<Vec<&str>, Error> {
        self.remove(key)
            .ok_or_else(|| format_err!("missing key: {:?}", key))
    }

    fn take_one_line(&mut self, key: &str) -> Result<&str, Error> {
        rfc822::one_line(&self.take_err(key)?)
    }

    fn remove_one_line<S: AsRef<str>>(&mut self, key: S) -> Result<Option<&str>, Error> {
        self.remove(key.as_ref())
            .map(|v| rfc822::one_line(&v))
            .inside_out()
    }

    fn get_if_one_line(&self, key: &str) -> Option<&str> {
        self.get(key).and_then(|v| rfc822::one_line(v).ok())
    }
}

impl<'s> RfcMapExt for HashMap<&'s str, Vec<&'s str>> {
    fn get(&self, key: &str) -> Option<&Vec<&str>> {
        HashMap::get(self, key)
    }
    fn remove(&mut self, key: &str) -> Option<Vec<&str>> {
        HashMap::remove(self, key)
    }
}

impl fmt::Display for Package {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)?;
        match self.arches.len() {
            0 => (),
            1 => write!(f, ":{}", self.arches.iter().next().unwrap())?,
            _ => unimplemented!("Don't know how to format multiple arches:\n{:?}", self),
        }
        write!(f, "={}", self.version)
    }
}

fn parse_dep(multi_str: &[&str]) -> Result<Vec<Dependency>, Error> {
    deps::read(&rfc822::joined(multi_str))
}

impl Constraint {
    pub fn new(operator: ConstraintOperator, version: &str) -> Self {
        Constraint {
            operator,
            version: version.to_string(),
        }
    }

    pub fn satisfied_by<S: AsRef<str>>(&self, version: S) -> bool {
        self.operator
            .satisfied_by(compare_versions(version.as_ref(), &self.version))
    }
}

impl ConstraintOperator {
    fn satisfied_by(&self, ordering: cmp::Ordering) -> bool {
        use self::ConstraintOperator::*;
        use std::cmp::Ordering::*;

        match *self {
            Eq => Equal == ordering,
            Ge => Less != ordering,
            Le => Greater != ordering,
            Lt => Less == ordering,
            Gt => Greater == ordering,
        }
    }
}

impl Default for PackageType {
    fn default() -> Self {
        PackageType::Binary(Binary::default())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::Constraint;
    use super::ConstraintOperator;
    use super::PackageType;

    const SOURCE_BLOCK_ALIEN: &str = r#"Package: alien-arena
Binary: alien-arena, alien-arena-server
Version: 7.66+dfsg-5
Maintainer: Debian Games Team <pkg-games-devel@lists.alioth.debian.org>
Uploaders: Michael Gilbert <mgilbert@debian.org>, Barry deFreese <bddebian@comcast.net>
Build-Depends: debhelper (>= 10), sharutils, libglu1-mesa-dev, libgl1-mesa-dev, libjpeg-dev, libpng-dev, libxxf86vm-dev, libxxf86dga-dev, libxext-dev, libx11-dev, libcurl4-gnutls-dev, libopenal-dev, libvorbis-dev, libfreetype6-dev, pkg-config
Architecture: any
Standards-Version: 4.0.1
Format: 3.0 (quilt)
Files:
 f26e5a6a298163277318a720b77a3b58 2291 alien-arena_7.66+dfsg-5.dsc
 af12838d2346b05a6e043141ceb40c49 1767600 alien-arena_7.66+dfsg.orig.tar.gz
 d806e404397c6338eae0d6470b4e8723 13844 alien-arena_7.66+dfsg-5.debian.tar.xz
Vcs-Browser: https://salsa.debian.org/games-team/alien-arena
Vcs-Git: https://salsa.debian.org/games-team/alien-arena.git
Checksums-Sha256:
 85eabee2877db5e070cd6549078ece3e5b4bc35a3a33ff8987d06fbb9732cd6e 2291 alien-arena_7.66+dfsg-5.dsc
 d4d173aba65fbdbf338e4fbdcb04a888e0cd3790e6de72597ba74b0bef42c14b 1767600 alien-arena_7.66+dfsg.orig.tar.gz
 6e90eabd98ac9c98ebe55b064ceb427101a3d4d4ff0b8aa4a2cea28052ec34c1 13844 alien-arena_7.66+dfsg-5.debian.tar.xz
Homepage: http://red.planetarena.org
Package-List:
 alien-arena deb contrib/games optional arch=any
 alien-arena-server deb contrib/games optional arch=any
Directory: pool/contrib/a/alien-arena
Priority: source
Section: contrib/games
"#;

    #[test]
    fn parse_alien() {
        let pkg = super::Package::parse(
            &mut super::rfc822::scan(SOURCE_BLOCK_ALIEN)
                .collect_to_map()
                .unwrap(),
        )
        .unwrap();

        assert_eq!("7.66+dfsg-5", pkg.version);

        let src = match pkg.style {
            PackageType::Source(s) => s,
            other => panic!("bad type: {:?}", other),
        };

        assert_eq!(
            vec!["alien-arena", "alien-arena-server"],
            src.binaries
                .into_iter()
                .map(|b| b.name)
                .collect::<Vec<String>>()
        );
        assert_eq!(HashMap::new(), pkg.unparsed);
    }

    const PROVIDES_EXAMPLE: &str = r#"Package: python3-cffi-backend
Status: install ok installed
Priority: optional
Section: python
Installed-Size: 190
Maintainer: Ubuntu Developers <ubuntu-devel-discuss@lists.ubuntu.com>
Architecture: amd64
Source: python-cffi
Version: 1.11.5-1
Replaces: python3-cffi (<< 1)
Provides: python3-cffi-backend-api-9729, python3-cffi-backend-api-max (= 10495), python3-cffi-backend-api-min (= 9729)
Depends: python3 (<< 3.7), python3 (>= 3.6~), python3:any (>= 3.1~), libc6 (>= 2.14), libffi6 (>= 3.0.4)
Breaks: python3-cffi (<< 1)
Description: Foreign Function Interface for Python 3 calling C code - runtime
 Convenient and reliable way of calling C code from Python 3.
 .
 The aim of this project is to provide a convenient and reliable way of calling
 C code from Python. It keeps Python logic in Python, and minimises the C
 required. It is able to work at either the C API or ABI level, unlike most
 other approaches, that only support the ABI level.
 .
 This package contains the runtime support for pre-built cffi modules.
Original-Maintainer: Debian Python Modules Team <python-modules-team@lists.alioth.debian.org>
Homepage: http://cffi.readthedocs.org/
"#;

    #[test]
    fn version() {
        let cons = Constraint::new(ConstraintOperator::Gt, "1.0");
        assert!(cons.satisfied_by("2.0"));
        assert!(!cons.satisfied_by("1.0"));
    }

    #[test]
    fn parse_provides() {
        let p = super::Package::parse(
            &mut super::rfc822::scan(PROVIDES_EXAMPLE)
                .collect_to_map()
                .unwrap(),
        )
        .unwrap();
        assert_eq!("python3-cffi-backend", p.name.as_str());
        let bin = match p.style {
            PackageType::Binary(bin) => bin,
            _ => panic!("wrong type!"),
        };
        assert_eq!(3, bin.provides.len());
        assert_eq!(HashMap::new(), p.unparsed);
    }
}
