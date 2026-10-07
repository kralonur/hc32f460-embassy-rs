#[doc = "Register `BCSTRL` reader"]
pub type R = crate::R<BcstrlSpec>;
#[doc = "Register `BCSTRL` writer"]
pub type W = crate::W<BcstrlSpec>;
#[doc = "Field `START` reader - desc START"]
pub type StartR = crate::BitReader;
#[doc = "Field `START` writer - desc START"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - desc DIR"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - desc DIR"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - desc MODE"]
pub type ModeR = crate::BitReader;
#[doc = "Field `MODE` writer - desc MODE"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNST` reader - desc SYNST"]
pub type SynstR = crate::BitReader;
#[doc = "Field `SYNST` writer - desc SYNST"]
pub type SynstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKDIV` reader - desc CKDIV"]
pub type CkdivR = crate::FieldReader;
#[doc = "Field `CKDIV` writer - desc CKDIV"]
pub type CkdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - desc DIR"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - desc SYNST"]
    #[inline(always)]
    pub fn synst(&self) -> SynstR {
        SynstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&self) -> CkdivR {
        CkdivR::new((self.bits >> 4) & 0x0f)
    }
}
impl W {
    #[doc = "Bit 0 - desc START"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, BcstrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - desc DIR"]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<'_, BcstrlSpec> {
        DirW::new(self, 1)
    }
    #[doc = "Bit 2 - desc MODE"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, BcstrlSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 3 - desc SYNST"]
    #[inline(always)]
    pub fn synst(&mut self) -> SynstW<'_, BcstrlSpec> {
        SynstW::new(self, 3)
    }
    #[doc = "Bits 4:7 - desc CKDIV"]
    #[inline(always)]
    pub fn ckdiv(&mut self) -> CkdivW<'_, BcstrlSpec> {
        CkdivW::new(self, 4)
    }
}
#[doc = "desc BCSTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`bcstrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcstrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BcstrlSpec;
impl crate::RegisterSpec for BcstrlSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bcstrl::R`](R) reader structure"]
impl crate::Readable for BcstrlSpec {}
#[doc = "`write(|w| ..)` method takes [`bcstrl::W`](W) writer structure"]
impl crate::Writable for BcstrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BCSTRL to value 0x02"]
impl crate::Resettable for BcstrlSpec {
    const RESET_VALUE: u8 = 0x02;
}
