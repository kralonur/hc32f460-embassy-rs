#[doc = "Register `CR0` reader"]
pub type R = crate::R<Cr0Spec>;
#[doc = "Register `CR0` writer"]
pub type W = crate::W<Cr0Spec>;
#[doc = "Field `MS` reader - desc MS"]
pub type MsR = crate::FieldReader;
#[doc = "Field `MS` writer - desc MS"]
pub type MsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `ACCSEL` reader - desc ACCSEL"]
pub type AccselR = crate::FieldReader;
#[doc = "Field `ACCSEL` writer - desc ACCSEL"]
pub type AccselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CLREN` reader - desc CLREN"]
pub type ClrenR = crate::BitReader;
#[doc = "Field `CLREN` writer - desc CLREN"]
pub type ClrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFMT` reader - desc DFMT"]
pub type DfmtR = crate::BitReader;
#[doc = "Field `DFMT` writer - desc DFMT"]
pub type DfmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AVCNT` reader - desc AVCNT"]
pub type AvcntR = crate::FieldReader;
#[doc = "Field `AVCNT` writer - desc AVCNT"]
pub type AvcntW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:1 - desc MS"]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - desc ACCSEL"]
    #[inline(always)]
    pub fn accsel(&self) -> AccselR {
        AccselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - desc CLREN"]
    #[inline(always)]
    pub fn clren(&self) -> ClrenR {
        ClrenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc DFMT"]
    #[inline(always)]
    pub fn dfmt(&self) -> DfmtR {
        DfmtR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - desc AVCNT"]
    #[inline(always)]
    pub fn avcnt(&self) -> AvcntR {
        AvcntR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - desc MS"]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<'_, Cr0Spec> {
        MsW::new(self, 0)
    }
    #[doc = "Bits 4:5 - desc ACCSEL"]
    #[inline(always)]
    pub fn accsel(&mut self) -> AccselW<'_, Cr0Spec> {
        AccselW::new(self, 4)
    }
    #[doc = "Bit 6 - desc CLREN"]
    #[inline(always)]
    pub fn clren(&mut self) -> ClrenW<'_, Cr0Spec> {
        ClrenW::new(self, 6)
    }
    #[doc = "Bit 7 - desc DFMT"]
    #[inline(always)]
    pub fn dfmt(&mut self) -> DfmtW<'_, Cr0Spec> {
        DfmtW::new(self, 7)
    }
    #[doc = "Bits 8:10 - desc AVCNT"]
    #[inline(always)]
    pub fn avcnt(&mut self) -> AvcntW<'_, Cr0Spec> {
        AvcntW::new(self, 8)
    }
}
#[doc = "desc CR0\n\nYou can [`read`](crate::Reg::read) this register and get [`cr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr0Spec;
impl crate::RegisterSpec for Cr0Spec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`cr0::R`](R) reader structure"]
impl crate::Readable for Cr0Spec {}
#[doc = "`write(|w| ..)` method takes [`cr0::W`](W) writer structure"]
impl crate::Writable for Cr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR0 to value 0"]
impl crate::Resettable for Cr0Spec {}
