#[doc = "Register `GCONR` reader"]
pub type R = crate::R<GconrSpec>;
#[doc = "Register `GCONR` writer"]
pub type W = crate::W<GconrSpec>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CKDIV` reader - desc CKDIV"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - desc CKDIV"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZMSKREV` reader - desc ZMSKREV"]
pub type ZmskrevR = crate::BitReader;
#[doc = "Field `ZMSKREV` writer - desc ZMSKREV"]
pub type ZmskrevW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZMSKPOS` reader - desc ZMSKPOS"]
pub type ZmskposR = crate::BitReader;
#[doc = "Field `ZMSKPOS` writer - desc ZMSKPOS"]
pub type ZmskposW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZMSKVAL` reader - desc ZMSKVAL"]
pub type ZmskvalR = crate::FieldReader;
#[doc = "Field `ZMSKVAL` writer - desc ZMSKVAL"]
pub type ZmskvalW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 8 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - desc ZMSKREV"]
    #[inline(always)]
    pub fn zmskrev(&self) -> ZmskrevR {
        ZmskrevR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - desc ZMSKPOS"]
    #[inline(always)]
    pub fn zmskpos(&self) -> ZmskposR {
        ZmskposR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:19 - desc ZMSKVAL"]
    #[inline(always)]
    pub fn zmskval(&self) -> ZmskvalR {
        ZmskvalR::new(((self.bits >> 18) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, GconrSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bits 1:3 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, GconrSpec> {
        ModeW::new(self, 1)
    }
    #[doc = "Bits 4:6 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<'_, GconrSpec> {
        CkdivW::new(self, 4)
    }
    #[doc = "Bit 8 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, GconrSpec> {
        DirW::new(self, 8)
    }
    #[doc = "Bit 16 - desc ZMSKREV"]
    #[inline(always)]
    pub fn zmskrev(&mut self) -> ZmskrevW<'_, GconrSpec> {
        ZmskrevW::new(self, 16)
    }
    #[doc = "Bit 17 - desc ZMSKPOS"]
    #[inline(always)]
    pub fn zmskpos(&mut self) -> ZmskposW<'_, GconrSpec> {
        ZmskposW::new(self, 17)
    }
    #[doc = "Bits 18:19 - desc ZMSKVAL"]
    #[inline(always)]
    pub fn zmskval(&mut self) -> ZmskvalW<'_, GconrSpec> {
        ZmskvalW::new(self, 18)
    }
}
#[doc = "desc GCONR\n\nYou can [`read`](crate::Reg::read) this register and get [`gconr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gconr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GconrSpec;
impl crate::RegisterSpec for GconrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gconr::R`](R) reader structure"]
impl crate::Readable for GconrSpec {}
#[doc = "`write(|w| ..)` method takes [`gconr::W`](W) writer structure"]
impl crate::Writable for GconrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCONR to value 0x0100"]
impl crate::Resettable for GconrSpec {
    const RESET_VALUE: u32 = 0x0100;
}
