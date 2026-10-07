#[doc = "Register `SCMRWL` reader"]
pub type R = crate::R<ScmrwlSpec>;
#[doc = "Register `SCMRWL` writer"]
pub type W = crate::W<ScmrwlSpec>;
#[doc = "Field `AMC` reader - desc AMC"]
pub type AmcR = crate::FieldReader;
#[doc = "Field `AMC` writer - desc AMC"]
pub type AmcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MZCE` reader - desc MZCE"]
pub type MzceR = crate::BitReader;
#[doc = "Field `MZCE` writer - desc MZCE"]
pub type MzceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MPCE` reader - desc MPCE"]
pub type MpceR = crate::BitReader;
#[doc = "Field `MPCE` writer - desc MPCE"]
pub type MpceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - desc AMC"]
    #[inline(always)]
    pub fn amc(&self) -> AmcR {
        AmcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 6 - desc MZCE"]
    #[inline(always)]
    pub fn mzce(&self) -> MzceR {
        MzceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc MPCE"]
    #[inline(always)]
    pub fn mpce(&self) -> MpceR {
        MpceR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - desc AMC"]
    #[inline(always)]
    pub fn amc(&mut self) -> AmcW<'_, ScmrwlSpec> {
        AmcW::new(self, 0)
    }
    #[doc = "Bit 6 - desc MZCE"]
    #[inline(always)]
    pub fn mzce(&mut self) -> MzceW<'_, ScmrwlSpec> {
        MzceW::new(self, 6)
    }
    #[doc = "Bit 7 - desc MPCE"]
    #[inline(always)]
    pub fn mpce(&mut self) -> MpceW<'_, ScmrwlSpec> {
        MpceW::new(self, 7)
    }
}
#[doc = "desc SCMRWL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrwl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrwl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmrwlSpec;
impl crate::RegisterSpec for ScmrwlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scmrwl::R`](R) reader structure"]
impl crate::Readable for ScmrwlSpec {}
#[doc = "`write(|w| ..)` method takes [`scmrwl::W`](W) writer structure"]
impl crate::Writable for ScmrwlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMRWL to value 0xff00"]
impl crate::Resettable for ScmrwlSpec {
    const RESET_VALUE: u16 = 0xff00;
}
