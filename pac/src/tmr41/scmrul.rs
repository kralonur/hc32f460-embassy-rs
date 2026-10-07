#[doc = "Register `SCMRUL` reader"]
pub type R = crate::R<ScmrulSpec>;
#[doc = "Register `SCMRUL` writer"]
pub type W = crate::W<ScmrulSpec>;
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
    pub fn amc(&mut self) -> AmcW<'_, ScmrulSpec> {
        AmcW::new(self, 0)
    }
    #[doc = "Bit 6 - desc MZCE"]
    #[inline(always)]
    pub fn mzce(&mut self) -> MzceW<'_, ScmrulSpec> {
        MzceW::new(self, 6)
    }
    #[doc = "Bit 7 - desc MPCE"]
    #[inline(always)]
    pub fn mpce(&mut self) -> MpceW<'_, ScmrulSpec> {
        MpceW::new(self, 7)
    }
}
#[doc = "desc SCMRUL\n\nYou can [`read`](crate::Reg::read) this register and get [`scmrul::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmrul::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScmrulSpec;
impl crate::RegisterSpec for ScmrulSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`scmrul::R`](R) reader structure"]
impl crate::Readable for ScmrulSpec {}
#[doc = "`write(|w| ..)` method takes [`scmrul::W`](W) writer structure"]
impl crate::Writable for ScmrulSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCMRUL to value 0xff00"]
impl crate::Resettable for ScmrulSpec {
    const RESET_VALUE: u16 = 0xff00;
}
